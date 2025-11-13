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
mod texture;
mod mesh;

use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect};
use camera::Camera;
use light::Light;
use material::{Material, vector3_to_color};
use cube::Cube;
use texture::TextureManager;
use mesh::Mesh;

const ORIGIN_BIAS: f32 = 1e-4;
const MAX_DEPTH: u32 = 2;

// TextureManager global (thread-safe)
lazy_static::lazy_static! {
    static ref TEXTURE_MANAGER: Arc<TextureManager> = Arc::new(TextureManager::new());
}

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

    for object in objects.iter() {
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

    // Aplicar textura si el material tiene una
    let mut diffuse_color = intersect.material.diffuse;
    if let Some(ref texture_name) = intersect.material.texture_path {
        let texture_color = TEXTURE_MANAGER.sample(texture_name, intersect.u, intersect.v);
        diffuse_color = texture_color;
    }

    let view_dir = (*ray_origin - intersect.point).normalized();
    let mut final_color = Vector3::zero();

    let ambient = if intersect.point.y < 0.0 {
        Vector3::new(0.05, 0.01, 0.01)
    } else {
        Vector3::new(0.12, 0.12, 0.15)
    };
    final_color = final_color + diffuse_color * ambient;

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

        let diffuse = diffuse_color * light_color_v3 * diffuse_dot * light_intensity;

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

fn create_scene_with_obj() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut objects: Vec<Arc<dyn RayIntersect + Send + Sync>> = Vec::new();

    // ========== MATERIALES CON TEXTURAS ==========
    
    let grass = Material::new(
        Vector3::new(0.2, 0.8, 0.2),
        10.0,
        [0.9, 0.1],
        0.0,
        0.0,
        Vector3::zero(),
        Some("grass_top".to_string()),
    );

    let dirt = Material::new(
        Vector3::new(0.6, 0.4, 0.2),
        5.0,
        [0.85, 0.05],
        0.0,
        0.0,
        Vector3::zero(),
        Some("dirt".to_string()),
    );

    let wood = Material::new(
        Vector3::new(0.4, 0.25, 0.1),
        8.0,
        [0.75, 0.08],
        0.0,
        0.0,
        Vector3::zero(),
        Some("wood".to_string()),
    );

    let stone = Material::new(
        Vector3::new(0.5, 0.5, 0.5),
        15.0,
        [0.8, 0.15],
        0.0,
        0.0,
        Vector3::zero(),
        Some("stone".to_string()),
    );

    // ========== CARGAR MODELOS OBJ ==========
    
    // Cubo de ejemplo con textura de piedra
    if let Ok(cube_mesh) = Mesh::from_obj(
        "assets/cube.obj",
        stone.clone(),
        Vector3::new(0.0, 3.0, 0.0),
        1.0,
    ) {
        objects.extend(cube_mesh.to_objects());
        println!("✅ Cubo OBJ cargado exitosamente");
    } else {
        println!("⚠️  No se pudo cargar cube.obj, usando cubo simple");
        objects.push(Arc::new(Cube::new(
            Vector3::new(0.0, 3.0, 0.0),
            2.0,
            stone.clone(),
        )));
    }

    // Segundo cubo con textura de madera
    if let Ok(wood_cube) = Mesh::from_obj(
        "assets/cube.obj",
        wood.clone(),
        Vector3::new(3.0, 2.0, 0.0),
        0.8,
    ) {
        objects.extend(wood_cube.to_objects());
        println!("✅ Cubo de madera cargado");
    }

    // ========== SUELO ==========
    for x in -5..=5 {
        for z in -5..=5 {
            objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, 0.0, z as f32),
                1.0,
                if (x + z) % 2 == 0 { grass.clone() } else { dirt.clone() },
            )));
        }
    }

    // ========== LUCES ==========
    let lights = vec![
        Light::new(
            Vector3::new(5.0, 10.0, 5.0),
            Color::new(255, 250, 240, 255),
            3.5,
        ),
        Light::new(
            Vector3::new(-5.0, 8.0, -5.0),
            Color::new(255, 200, 150, 255),
            2.0,
        ),
    ];

    println!("🎨 Escena creada: {} objetos", objects.len());
    (objects, lights)
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("🎮 Ray Tracer con Modelos OBJ y Texturas")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    println!("📦 Cargando escena con modelos OBJ...");
    let (objects, lights) = create_scene_with_obj();

    let mut camera = Camera::new(
        Vector3::new(8.0, 6.0, 8.0),
        Vector3::new(0.0, 2.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI / 60.0;
    let zoom_speed = 0.3;

    println!("🎨 Renderizando escena inicial...");
    let start = std::time::Instant::now();
    render(&mut framebuffer, &objects, &camera, &lights);
    println!("✨ Renderizado en {:.2}s", start.elapsed().as_secs_f32());

    window.set_target_fps(30);

    println!("\n🎮 Controles:");
    println!("  ← → : Rotar horizontalmente");
    println!("  ↑ ↓ : Rotar verticalmente");
    println!("  W S : Zoom in/out");
    println!("  ESC : Salir");
    println!("\n✨ Observa las texturas procedurales en los cubos OBJ!");

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