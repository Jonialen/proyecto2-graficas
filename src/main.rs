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
const MAX_DEPTH: u32 = 1; // Reducido para mejor rendimiento

fn sky_color(dir: Vector3, is_nether: bool) -> Vector3 {
    let d = dir.normalized();
    let t = (d.y + 1.0) * 0.5;
    
    if is_nether {
        // Cielo del Nether - tonos rojos oscuros
        let dark_red = Vector3::new(0.2, 0.02, 0.02);
        let deep_crimson = Vector3::new(0.35, 0.05, 0.05);
        
        if t < 0.5 {
            dark_red
        } else {
            let k = (t - 0.5) / 0.5;
            dark_red * (1.0 - k) + deep_crimson * k
        }
    } else {
        // Cielo del Overworld - verde césped y azul
        let grass_green = Vector3::new(0.3, 0.6, 0.3);
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

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            return 0.8;
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

    // Luz ambiental diferente según la dimensión
    let ambient = if intersect.point.y < 0.0 {
        Vector3::new(0.08, 0.02, 0.02) // Nether
    } else {
        Vector3::new(0.15, 0.15, 0.18) // Overworld
    };
    final_color = final_color + intersect.material.diffuse * ambient;

    // Emisión del material
    if intersect.material.emissive.length() > 0.0 {
        final_color = final_color + intersect.material.emissive;
    }

    // Iluminación por cada luz
    for light in lights {
        let light_dir = (light.position - intersect.point).normalized();
        let diffuse_dot = intersect.normal.dot(light_dir);
        
        if diffuse_dot <= 0.0 {
            continue; // La luz está detrás de la superficie
        }

        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

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

    // Reflexión
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

    // Renderizado paralelo por filas
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

    // Escribir pixels al framebuffer
    for (i, color) in pixels.iter().enumerate() {
        let x = (i % width) as u32;
        let y = (i / width) as u32;
        framebuffer.set_current_color(*color);
        framebuffer.set_pixel(x, y);
    }
}

fn create_mirror_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut objects: Vec<Arc<dyn RayIntersect + Send + Sync>> = Vec::new();

    // ========== MATERIALES ==========
    
    // Overworld
    let grass = Material::new(
        Vector3::new(0.3, 0.7, 0.3),
        10.0,
        [0.8, 0.1],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let stone = Material::new(
        Vector3::new(0.5, 0.5, 0.5),
        15.0,
        [0.7, 0.15],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let wood = Material::new(
        Vector3::new(0.55, 0.35, 0.2),
        8.0,
        [0.75, 0.08],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let leaves = Material::new(
        Vector3::new(0.2, 0.6, 0.2),
        5.0,
        [0.85, 0.05],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let water = Material::new(
        Vector3::new(0.2, 0.4, 0.8),
        80.0,
        [0.3, 0.5],
        0.0,
        0.4,
        Vector3::zero(),
        None,
    );

    // Nether
    let netherrack = Material::new(
        Vector3::new(0.45, 0.15, 0.15),
        10.0,
        [0.8, 0.1],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let dark_wood = Material::new(
        Vector3::new(0.25, 0.15, 0.1),
        8.0,
        [0.7, 0.08],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let nether_leaves = Material::new(
        Vector3::new(0.6, 0.1, 0.1),
        5.0,
        [0.8, 0.05],
        0.0,
        0.0,
        Vector3::zero(),
        None,
    );

    let lava = Material::new(
        Vector3::new(1.0, 0.4, 0.0),
        20.0,
        [0.6, 0.3],
        0.0,
        0.0,
        Vector3::new(1.2, 0.5, 0.1),
        None,
    );

    // Emisivos
    let torch = Material::new(
        Vector3::new(0.9, 0.5, 0.1),
        40.0,
        [0.3, 0.2],
        0.0,
        0.0,
        Vector3::new(2.5, 1.2, 0.3),
        None,
    );

    // Plano espejo en Y=0
    let mirror = Material::new(
        Vector3::new(0.85, 0.85, 0.95),
        120.0,
        [0.1, 0.5],
        0.0,
        0.75,
        Vector3::zero(),
        None,
    );

    // ========== OVERWORLD (Y > 0) - 13x13 ==========
    
    // Piso de césped compacto (sin espacios)
    for x in -6..=6 {
        for z in -6..=6 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, 1.0, z as f32),
                1.0,
                grass.clone(),
            )));
        }
    }

    // Paredes de piedra
    for y in 2..=5 {
        for x in -6..=6 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, y as f32, -6.0),
                1.0,
                stone.clone(),
            )));
        }
        
        objects.push(Arc::new(Cube::new(
            Vector3::new(-6.0, y as f32, 0.0),
            1.0,
            stone.clone(),
        )));
        objects.push(Arc::new(Cube::new(
            Vector3::new(6.0, y as f32, 0.0),
            1.0,
            stone.clone(),
        )));
    }

    // Árbol del Overworld (izquierda) - Tronco
    for y in 2..=5 {
        objects.push(Arc::new(Cube::new(
            Vector3::new(-3.0, y as f32, -3.0),
            1.0,
            wood.clone(),
        )));
    }
    
    // Copa del árbol (hojas)
    for x in -4..=-2 {
        for z in -4..=-2 {
            for y in 5..=7 {
                if !(x == -3 && z == -3 && y == 5) {
                    objects.push(Arc::new(Cube::new(
                        Vector3::new(x as f32, y as f32, z as f32),
                        1.0,
                        leaves.clone(),
                    )));
                }
            }
        }
    }

    // Lago de agua (derecha)
    for x in 2..=4 {
        for z in -4..=-2 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, 1.0, z as f32),
                1.0,
                water.clone(),
            )));
        }
    }

    // Antorcha del Overworld
    objects.push(Arc::new(Cube::new(
        Vector3::new(0.0, 2.0, -4.0),
        0.3,
        torch.clone(),
    )));

    // ========== PLANO ESPEJO (Y=0) ==========
    for x in -7..=7 {
        for z in -7..=7 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, 0.0, z as f32),
                1.0,
                mirror.clone(),
            )));
        }
    }

    // ========== NETHER (Y < 0) - REFLEJO ==========
    
    // Techo de netherrack
    for x in -6..=6 {
        for z in -6..=6 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, -1.0, z as f32),
                1.0,
                netherrack.clone(),
            )));
        }
    }

    // Paredes oscuras
    for y in -5..=-2 {
        for x in -6..=6 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, y as f32, -6.0),
                1.0,
                netherrack.clone(),
            )));
        }
        
        objects.push(Arc::new(Cube::new(
            Vector3::new(-6.0, y as f32, 0.0),
            1.0,
            netherrack.clone(),
        )));
        objects.push(Arc::new(Cube::new(
            Vector3::new(6.0, y as f32, 0.0),
            1.0,
            netherrack.clone(),
        )));
    }

    // Árbol del Nether (izquierda) - Tronco oscuro
    for y in -5..=-2 {
        objects.push(Arc::new(Cube::new(
            Vector3::new(-3.0, y as f32, -3.0),
            1.0,
            dark_wood.clone(),
        )));
    }
    
    // Copa del árbol del Nether (hojas rojas)
    for x in -4..=-2 {
        for z in -4..=-2 {
            for y in -7..=-5 {
                if !(x == -3 && z == -3 && y == -5) {
                    objects.push(Arc::new(Cube::new(
                        Vector3::new(x as f32, y as f32, z as f32),
                        1.0,
                        nether_leaves.clone(),
                    )));
                }
            }
        }
    }

    // Lago de lava (derecha)
    for x in 2..=4 {
        for z in -4..=-2 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, -1.0, z as f32),
                1.0,
                lava.clone(),
            )));
        }
    }

    // Antorcha del Nether
    objects.push(Arc::new(Cube::new(
        Vector3::new(0.0, -2.0, -4.0),
        0.3,
        torch.clone(),
    )));

    // ========== LUCES ==========
    let lights = vec![
        // Luz del sol (Overworld - general)
        Light::new(
            Vector3::new(0.0, 15.0, 0.0),
            Color::new(255, 250, 240, 255),
            5.0,
        ),
        
        // Antorcha Overworld
        Light::new(
            Vector3::new(0.0, 2.5, -4.0),
            Color::new(255, 180, 80, 255),
            4.0,
        ),
        
        // Lagos de lava (múltiples luces)
        Light::new(
            Vector3::new(2.5, -0.5, -3.0),
            Color::new(255, 120, 30, 255),
            5.0,
        ),
        Light::new(
            Vector3::new(3.5, -0.5, -3.0),
            Color::new(255, 100, 20, 255),
            4.5,
        ),
        
        // Antorcha Nether
        Light::new(
            Vector3::new(0.0, -2.0, -4.0),
            Color::new(255, 140, 50, 255),
            4.0,
        ),
    ];

    (objects, lights)
}

fn main() {
    let window_width = 1000;
    let window_height = 700;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Mirror Worlds - Overworld & Nether")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    let (objects, lights) = create_mirror_scene();

    let mut camera = Camera::new(
        Vector3::new(0.0, 2.0, 12.0),
        Vector3::new(0.0, 0.0, -2.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI / 80.0;
    let zoom_speed = 0.25;

    println!("Renderizando escena inicial (esto puede tomar unos segundos)...");
    let start = std::time::Instant::now();
    render(&mut framebuffer, &objects, &camera, &lights);
    println!("Renderizado completo en {:.2}s", start.elapsed().as_secs_f32());

    window.set_target_fps(60);

    println!("\nControles:");
    println!("  ← → : Rotar horizontalmente");
    println!("  ↑ ↓ : Rotar verticalmente");
    println!("  W S : Zoom in/out");
    println!("  ESC : Salir");
    println!("\nMira el espejo en Y=0 que refleja ambos mundos!");

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
            if elapsed > 0.1 {
                println!("Frame renderizado en {:.3}s ({:.1} FPS)", elapsed, 1.0 / elapsed);
            }
        }

        framebuffer.swap_buffers(&mut window, &thread);
    }
}