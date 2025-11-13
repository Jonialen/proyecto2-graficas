use raylib::prelude::*;
use rayon::prelude::*;
use std::f32::consts::PI;
use std::sync::Arc;

mod framebuffer;
mod ray_intersect;
mod camera;
mod light;
mod material;
mod cube;

use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect};
use camera::Camera;
use light::Light;
use material::{Material, vector3_to_color};
use cube::Cube;

const ORIGIN_BIAS: f32 = 1e-4;
const MAX_DEPTH: u32 = 2;

fn sky_color(dir: Vector3, is_nether: bool) -> Vector3 {
    let d = dir.normalized();
    let t = (d.y + 1.0) * 0.5;
    
    if is_nether {
        let dark_red = Vector3::new(0.15, 0.01, 0.01);
        let deep_crimson = Vector3::new(0.25, 0.03, 0.03);
        
        if t < 0.5 {
            dark_red
        } else {
            let k = (t - 0.5) / 0.5;
            dark_red * (1.0 - k) + deep_crimson * k
        }
    } else {
        let grass_green = Vector3::new(0.4, 0.7, 0.4);
        let sky_blue = Vector3::new(0.5, 0.7, 1.0);
        
        if t < 0.45 {
            grass_green
        } else if t < 0.55 {
            let k = (t - 0.45) / 0.1;
            grass_green * (1.0 - k) + sky_blue * k
        } else {
            sky_blue
        }
    }
}

#[inline]
fn offset_origin(intersect: &Intersect, direction: &Vector3) -> Vector3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

#[inline]
fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Arc<dyn RayIntersect + Send + Sync>],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalized();
    let light_distance = (light.position - intersect.point).length();
    let shadow_ray_origin = offset_origin(intersect, &light_dir);

    for object in objects.iter().take(30) {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            return 0.6;
        }
    }
    0.0
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Arc<dyn RayIntersect + Send + Sync>],
    lights: &[Light],
    depth: u32,
) -> Vector3 {
    if depth > MAX_DEPTH {
        let is_nether = ray_origin.y < 0.0;
        return sky_color(*ray_direction, is_nether);
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        let is_nether = ray_origin.y < 0.0;
        return sky_color(*ray_direction, is_nether);
    }

    let view_dir = (*ray_origin - intersect.point).normalized();
    let mut final_color = Vector3::zero();

    let ambient = if intersect.point.y < 0.0 {
        Vector3::new(0.05, 0.01, 0.01)
    } else {
        Vector3::new(0.12, 0.12, 0.15)
    };
    final_color = final_color + intersect.material.diffuse * ambient;

    if intersect.material.emissive.length() > 0.0 {
        final_color = final_color + intersect.material.emissive * 0.5;
    }

    for light in lights {
        let light_dir = (light.position - intersect.point).normalized();
        let diffuse_dot = intersect.normal.dot(light_dir);
        
        if diffuse_dot <= 0.0 {
            continue;
        }

        let distance = (light.position - intersect.point).length();
        let attenuation = 1.0 / (1.0 + 0.05 * distance + 0.01 * distance * distance);

        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity) * attenuation;

        let light_color_v3 = Vector3::new(
            light.color.r as f32 / 255.0,
            light.color.g as f32 / 255.0,
            light.color.b as f32 / 255.0,
        );

        let diffuse = intersect.material.diffuse * light_color_v3 * diffuse_dot * light_intensity;

        let reflect_dir = reflect(&-light_dir, &intersect.normal);
        let specular_intensity = view_dir
            .dot(reflect_dir)
            .max(0.0)
            .powf(intersect.material.specular)
            * light_intensity;
        let specular = light_color_v3 * specular_intensity;

        final_color = final_color + diffuse * intersect.material.albedo[0] 
                                  + specular * intersect.material.albedo[1];
    }

    let reflectivity = intersect.material.reflectivity;
    if reflectivity > 0.05 && depth < MAX_DEPTH {
        let reflect_dir = reflect(ray_direction, &intersect.normal).normalized();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        let reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1);
        final_color = final_color * (1.0 - reflectivity) + reflect_color * reflectivity;
    }

    final_color
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Arc<dyn RayIntersect + Send + Sync>],
    camera: &Camera,
    lights: &[Light],
) {
    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;
    let aspect_ratio = width as f32 / height as f32;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    let pixels: Vec<Color> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            let mut row_colors = Vec::with_capacity(width);
            for x in 0..width {
                let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
                let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;

                let screen_x = screen_x * aspect_ratio * perspective_scale;
                let screen_y = screen_y * perspective_scale;

                let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();
                let rotated_direction = camera.basis_change(&ray_direction);

                let pixel_color_v3 = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0);
                let pixel_color = vector3_to_color(pixel_color_v3);

                row_colors.push(pixel_color);
            }
            row_colors
        })
        .collect();

    for (i, color) in pixels.iter().enumerate() {
        let x = (i % width) as u32;
        let y = (i / width) as u32;
        framebuffer.set_current_color(*color);
        framebuffer.set_pixel(x, y);
    }
}

fn create_island_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut objects: Vec<Arc<dyn RayIntersect + Send + Sync>> = Vec::new();

    // ========== MATERIALES ==========
    
    // Overworld - colores más saturados y distintos
    let grass = Material::new(
        Vector3::new(0.2, 0.8, 0.2),  // Verde brillante
        10.0,
        [0.9, 0.1],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let dirt = Material::new(
        Vector3::new(0.6, 0.4, 0.2),  // Café tierra
        5.0,
        [0.85, 0.05],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let wood = Material::new(
        Vector3::new(0.4, 0.25, 0.1),  // Café madera oscura
        8.0,
        [0.75, 0.08],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let leaves = Material::new(
        Vector3::new(0.1, 0.5, 0.1),  // Verde oscuro hojas
        5.0,
        [0.85, 0.05],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let water = Material::new(
        Vector3::new(0.1, 0.3, 0.7),  // Azul profundo
        100.0,
        [0.2, 0.6],
        0.0,
        0.5,  // Más reflectante
        Vector3::zero(),
        None,
    );

    let torch = Material::new(
        Vector3::new(1.0, 0.6, 0.0),  // Naranja brillante
        40.0,
        [0.3, 0.2],
        0.0,
        0.0,
        Vector3::new(1.2, 0.6, 0.1),
        None,
    );

    // Nether - colores más dramáticos
    let netherrack = Material::new(
        Vector3::new(0.6, 0.1, 0.1),  // Rojo netherrack
        10.0,
        [0.8, 0.1],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let nether_brick = Material::new(
        Vector3::new(0.2, 0.05, 0.05),  // Muy oscuro
        15.0,
        [0.7, 0.15],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let lava = Material::new(
        Vector3::new(1.0, 0.3, 0.0),  // Naranja lava
        20.0,
        [0.6, 0.3],
        0.0,
        0.0,
        Vector3::new(1.0, 0.4, 0.05),
        None,
    );

    let soul_sand = Material::new(
        Vector3::new(0.3, 0.2, 0.15),  // Café soul sand
        8.0,
        [0.75, 0.1],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    // Plano espejo
    let mirror = Material::new(
        Vector3::new(0.9, 0.9, 1.0),
        120.0,
        [0.05, 0.6],
        0.0,
        0.85,
        Vector3::zero(),
        None,
    );

    // ========== ISLA FLOTANTE (OVERWORLD) Y > 0 ==========
    
    // Base de la isla (forma circular 7x7)
    let island_blocks = vec![
        // Centro (y = 1-2)
        (0, 0), (1, 0), (-1, 0), (0, 1), (0, -1),
        (1, 1), (1, -1), (-1, 1), (-1, -1),
        (2, 0), (-2, 0), (0, 2), (0, -2),
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2),
        // Bordes
        (3, 0), (-3, 0), (0, 3), (0, -3),
        (2, 2), (2, -2), (-2, 2), (-2, -2),
    ];

    // Capa de césped (y = 2) - EXCEPTO donde irá el agua
    for (x, z) in &island_blocks {
        // Saltar posiciones del laguito (centro de la isla)
        if !(*x >= -1 && *x <= 0 && *z >= -1 && *z <= 0) {
            objects.push(Arc::new(Cube::new(
                Vector3::new(*x as f32, 2.0, *z as f32),
                1.0,
                grass.clone(),
            )));
        }
    }

    // Capa de tierra (y = 1)
    for (x, z) in &island_blocks {
        objects.push(Arc::new(Cube::new(
            Vector3::new(*x as f32, 1.0, *z as f32),
            1.0,
            dirt.clone(),
        )));
    }

    // LAGUITO INTERIOR (2x2 en el centro, nivel más bajo)
    // Fondo de tierra visible
    for x in -1..=0 {
        for z in -1..=0 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, 1.5, z as f32),
                1.0,
                dirt.clone(),
            )));
        }
    }
    
    // Agua encima (nivel 1.8 para que se vea el hoyo)
    for x in -1..=0 {
        for z in -1..=0 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, 1.8, z as f32),
                1.0,
                water.clone(),
            )));
        }
    }
    // Tronco
    for y in 3..=5 {
        objects.push(Arc::new(Cube::new(
            Vector3::new(-2.0, y as f32, 0.0),
            1.0,
            wood.clone(),
        )));
    }

    // Copa del árbol (hojas en forma de cruz)
    let leaf_positions = vec![
        // Nivel 1 (y=5)
        (-3, 5, 0), (-2, 5, 1), (-2, 5, -1), (-1, 5, 0),
        // Nivel 2 (y=6)
        (-3, 6, 0), (-2, 6, 0), (-1, 6, 0),
        (-2, 6, 1), (-2, 6, -1),
        // Top (y=7)
        (-2, 7, 0),
    ];

    for (x, y, z) in leaf_positions {
        objects.push(Arc::new(Cube::new(
            Vector3::new(x as f32, y as f32, z as f32),
            1.0,
            leaves.clone(),
        )));
    }

    // LAGUITO (2x2 en posición: 1, -1)
    for x in 1..=2 {
        for z in -2..=-1 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, 2.0, z as f32),
                1.0,
                water.clone(),
            )));
        }
    }

    // ANTORCHA (al lado del árbol)
    objects.push(Arc::new(Cube::new(
        Vector3::new(-2.0, 3.0, 2.0),
        0.3,
        torch.clone(),
    )));

    // ========== PLANO ESPEJO (Y = 0) ==========
    for x in -5..=5 {
        for z in -5..=5 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, 0.0, z as f32),
                1.0,
                mirror.clone(),
            )));
        }
    }

    // ========== NETHER REFLEJADO (Y < 0) ==========
    
    // "Isla" del Nether (techo netherrack) - EXCEPTO donde irá la lava
    for (x, z) in &island_blocks {
        if !(*x >= -1 && *x <= 0 && *z >= -1 && *z <= 0) {
            objects.push(Arc::new(Cube::new(
                Vector3::new(*x as f32, -2.0, *z as f32),
                1.0,
                netherrack.clone(),
            )));
        }
        objects.push(Arc::new(Cube::new(
            Vector3::new(*x as f32, -1.0, *z as f32),
            1.0,
            nether_brick.clone(),
        )));
    }

    // Pozo de LAVA (reflejo del agua)
    for x in -1..=0 {
        for z in -1..=0 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, -1.5, z as f32),
                1.0,
                nether_brick.clone(),
            )));
        }
    }
    
    for x in -1..=0 {
        for z in -1..=0 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, -1.8, z as f32),
                1.0,
                lava.clone(),
            )));
        }
    }

    // "Árbol" del Nether (columna de netherrack)
    for y in -5..=-3 {
        objects.push(Arc::new(Cube::new(
            Vector3::new(-2.0, y as f32, 0.0),
            1.0,
            nether_brick.clone(),
        )));
    }

    // "Hojas" del Nether (soul sand)
    let nether_leaf_pos = vec![
        (-3, -5, 0), (-2, -5, 1), (-2, -5, -1), (-1, -5, 0),
        (-3, -6, 0), (-2, -6, 0), (-1, -6, 0),
        (-2, -6, 1), (-2, -6, -1),
        (-2, -7, 0),
    ];

    for (x, y, z) in nether_leaf_pos {
        objects.push(Arc::new(Cube::new(
            Vector3::new(x as f32, y as f32, z as f32),
            1.0,
            soul_sand.clone(),
        )));
    }

    // Lago de LAVA (mismo lugar que el agua)
    for x in 1..=2 {
        for z in -2..=-1 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, -2.0, z as f32),
                1.0,
                lava.clone(),
            )));
        }
    }

    // "Antorcha" del Nether (bloque emisivo)
    objects.push(Arc::new(Cube::new(
        Vector3::new(1.0, -3.0, 0.0),
        0.4,
        lava.clone(),
    )));

    // ========== LUCES ==========
    let lights = vec![
        // Luz solar (Overworld)
        Light::new(
            Vector3::new(5.0, 15.0, 5.0),
            Color::new(255, 250, 240, 255),
            3.0,
        ),
        
        // Antorcha del Overworld
        Light::new(
            Vector3::new(1.0, 3.5, 0.0),
            Color::new(255, 180, 80, 255),
            2.5,
        ),
        
        // Lago de lava (Nether)
        Light::new(
            Vector3::new(-0.5, -1.5, -0.5),
            Color::new(255, 100, 20, 255),
            3.5,
        ),
        
        // "Antorcha" del Nether
        Light::new(
            Vector3::new(1.0, -3.0, 0.0),
            Color::new(255, 120, 30, 255),
            2.0,
        ),
        
        // Luz ambiental del Nether
        Light::new(
            Vector3::new(0.0, -10.0, 0.0),
            Color::new(200, 50, 30, 255),
            1.5,
        ),
    ];

    println!("🏝️  Objetos en la isla: {}", objects.len());
    (objects, lights)
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("🏝️ Isla Flotante - Overworld & Nether Reflejado")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    let (objects, lights) = create_island_scene();

    let mut camera = Camera::new(
        Vector3::new(8.0, 4.0, 8.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI / 60.0;
    let zoom_speed = 0.3;

    println!("🎨 Renderizando isla flotante...");
    let start = std::time::Instant::now();
    render(&mut framebuffer, &objects, &camera, &lights);
    println!("✨ Renderizado en {:.2}s", start.elapsed().as_secs_f32());

    window.set_target_fps(30);

    println!("\n🎮 Controles:");
    println!("  ← → : Rotar horizontalmente");
    println!("  ↑ ↓ : Rotar verticalmente");
    println!("  W S : Zoom in/out");
    println!("  ESC : Salir");
    println!("\n✨ Observa el espejo en Y=0 que refleja el Nether!");

    while !window.window_should_close() {
        let mut needs_render = false;

        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.orbit(rotation_speed, 0.0);
            needs_render = true;
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.orbit(-rotation_speed, 0.0);
            needs_render = true;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            camera.orbit(0.0, -rotation_speed);
            needs_render = true;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.orbit(0.0, rotation_speed);
            needs_render = true;
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            camera.zoom(zoom_speed);
            needs_render = true;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            camera.zoom(-zoom_speed);
            needs_render = true;
        }

        if needs_render {
            let start = std::time::Instant::now();
            render(&mut framebuffer, &objects, &camera, &lights);
            let elapsed = start.elapsed().as_secs_f32();
            println!("⚡ Frame: {:.3}s ({:.1} FPS)", elapsed, 1.0 / elapsed);
        }

        framebuffer.swap_buffers(&mut window, &thread);
    }
}