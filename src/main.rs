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
mod scene_builder;

use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect, BVH};
use camera::Camera;
use light::Light;
use material::{Material, vector3_to_color};
use texture::TextureManager;
use crate::scene_builder::{SceneBuilder, WallDirection};

const ORIGIN_BIAS: f32 = 1e-4;
const MAX_DEPTH: u32 = 2;

lazy_static::lazy_static! {
    static ref TEXTURE_MANAGER: Arc<TextureManager> = Arc::new(TextureManager::new());
}

// === ESCENAS PREDEFINIDAS ===

fn create_simple_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .add_floor(10, "grass_top")
        .add_cube(0.0, 1.0, 0.0, 2.0, "stone")
        .add_sun(10.0, 15.0, 10.0, 3.0)
        .build()
}

fn create_house_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .add_checkered_floor(10, "grass_top", "dirt")
        .add_house(0, 0)
        .add_tree(-5, -5)
        .add_tree(-5, 5)
        .add_tree(8, -5)
        .add_tree(8, 5)
        .add_sun(15.0, 20.0, 15.0, 3.5)
        .build()
}

fn create_castle_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .add_floor(20, "stone")
        .add_tower(-10, -10, 8, "stone")
        .add_tower(-10, 10, 8, "stone")
        .add_tower(10, -10, 8, "stone")
        .add_tower(10, 10, 8, "stone")
        .add_wall(-10, -10, 21, 5, WallDirection::North, "stone")
        .add_wall(-10, 10, 21, 5, WallDirection::South, "stone")
        .add_wall(-10, -10, 21, 5, WallDirection::East, "stone")
        .add_wall(10, -10, 21, 5, WallDirection::West, "stone")
        .add_torches(&[
            (-5.0, 5.0, -10.0),
            (0.0, 5.0, -10.0),
            (5.0, 5.0, -10.0),
        ])
        .add_sun(20.0, 25.0, 20.0, 4.0)
        .build()
}

// === ESCENAS DE ISLA FLOTANTE ===

fn create_floating_island_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    println!("🏝️  Generando isla flotante básica...");
    let center_x = 0;
    let center_y = 10;
    let center_z = 0;
    let radius = 6;
    
    SceneBuilder::new()
        .add_floating_island(center_x, center_y, center_z, radius)
        .add_island_vegetation(center_x, center_y, center_z, radius)
        .add_nether_reflection(center_x, -center_y, center_z, radius)
        .add_nether_features(center_x, -center_y, center_z, radius)
        .add_checkered_floor(3, "glass", "stone")
        .add_dual_world_lighting(center_x as f32, center_z as f32)
        .build()
}

fn create_floating_island_with_waterfalls() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    println!("🏝️  Generando isla flotante con cascadas...");
    let center_x = 0;
    let center_y = 12;
    let center_z = 0;
    let radius = 7;
    
    let mut builder = SceneBuilder::new();
    
    builder = builder.add_floating_island(center_x, center_y, center_z, radius);
    builder = builder.add_island_vegetation(center_x, center_y, center_z, radius);
    
    println!("   💧 Añadiendo cascadas...");
    // Cascadas de agua cayendo desde los bordes
    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0;
        let x = center_x as f32 + (radius as f32 * 0.8) * angle.cos();
        let z = center_z as f32 + (radius as f32 * 0.8) * angle.sin();
        
        for h in 0..10 {
            builder = builder.add_cube(x, (center_y + radius - h * 2) as f32, z, 0.3, "water");
        }
    }
    
    builder = builder.add_nether_reflection(center_x, -center_y, center_z, radius);
    builder = builder.add_nether_features(center_x, -center_y, center_z, radius);
    builder = builder.add_dual_world_lighting(center_x as f32, center_z as f32);
    
    builder.build()
}

fn create_floating_island_with_bridge() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    println!("🏝️  Generando isla flotante con puente portal...");
    let center_x = 0;
    let center_y = 10;
    let center_z = 0;
    let radius = 6;
    
    let mut builder = SceneBuilder::new();
    
    builder = builder.add_floating_island(center_x, center_y, center_z, radius);
    builder = builder.add_island_vegetation(center_x, center_y, center_z, radius);
    builder = builder.add_nether_reflection(center_x, -center_y, center_z, radius);
    builder = builder.add_nether_features(center_x, -center_y, center_z, radius);
    
    // Puente vertical conectando ambos mundos (portal)
    println!("   🌉 Construyendo puente portal...");
    for y in (-center_y + radius)..(center_y - radius) {
        builder = builder
            .add_cube(center_x as f32 - 1.0, y as f32, center_z as f32, 0.3, "glass")
            .add_cube(center_x as f32 + 1.0, y as f32, center_z as f32, 0.3, "glass");
        
        // Partículas de portal cada 2 bloques
        if y % 2 == 0 {
            builder = builder.add_cube(center_x as f32, y as f32, center_z as f32, 0.2, "glowstone");
        }
    }
    
    builder = builder.add_dual_world_lighting(center_x as f32, center_z as f32);
    
    builder.build()
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
    bvh: &BVH,
    objects: &[Arc<dyn RayIntersect + Send + Sync>],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalized();
    let light_distance = (light.position - intersect.point).length();
    let shadow_ray_origin = offset_origin(intersect, &light_dir);

    let shadow_intersect = bvh.intersect(&shadow_ray_origin, &light_dir, objects);
    
    if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
        return 0.6;
    }
    0.0
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    bvh: &BVH,
    objects: &[Arc<dyn RayIntersect + Send + Sync>],
    lights: &[Light],
    depth: u32,
) -> Vector3 {
    if depth > MAX_DEPTH {
        let is_nether = ray_origin.y < 0.0;
        return sky_color(*ray_direction, is_nether);
    }

    let intersect = bvh.intersect(ray_origin, ray_direction, objects);

    if !intersect.is_intersecting {
        let is_nether = ray_origin.y < 0.0;
        return sky_color(*ray_direction, is_nether);
    }

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

        let shadow_intensity = cast_shadow(&intersect, light, bvh, objects);
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
        let reflect_color = cast_ray(&reflect_origin, &reflect_dir, bvh, objects, lights, depth + 1);
        final_color = final_color * (1.0 - reflectivity) + reflect_color * reflectivity;
    }

    final_color
}

pub fn render(
    framebuffer: &mut Framebuffer,
    bvh: &BVH,
    objects: &[Arc<dyn RayIntersect + Send + Sync>],
    camera: &Camera,
    lights: &[Light],
) {
    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;
    let aspect_ratio = width as f32 / height as f32;
    let fov = PI / 2.0; // 90 grados - FOV más amplio para ver más escena
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

                let pixel_color_v3 = cast_ray(&camera.eye, &rotated_direction, bvh, objects, lights, 0);
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

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("🏝️ Ray Tracer - Isla Flotante con Reflejo Nether")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    println!("\n╔════════════════════════════════════════╗");
    println!("║   🏝️  RAY TRACER - ISLA FLOTANTE  🏝️   ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("📦 Selecciona tu escena:\n");
    println!("1. 🏝️  Isla Flotante Básica");
    println!("2. 💧 Isla con Cascadas");
    println!("3. 🌉 Isla con Puente Portal");
    println!("4. 🏰 Castillo");
    println!("5. 🏠 Casa con Jardín");
    println!("6. 📦 Escena Simple\n");

    // CAMBIA ESTE NÚMERO PARA ELEGIR LA ESCENA
    let scene_choice = 4;

    let start = std::time::Instant::now();
    
    let (objects, lights) = match scene_choice {
        1 => create_floating_island_scene(),
        2 => create_floating_island_with_waterfalls(),
        3 => create_floating_island_with_bridge(),
        4 => create_castle_scene(),
        5 => create_house_scene(),
        _ => create_simple_scene(),
    };
    
    println!("\n⚡ Construyendo BVH para {} objetos...", objects.len());
    let bvh_start = std::time::Instant::now();
    let bvh = BVH::build(&objects);
    println!("✅ BVH construido en {:.3}s", bvh_start.elapsed().as_secs_f32());
    
    // Debug: mostrar bounds de la escena
    if !objects.is_empty() {
        let first_bounds = objects[0].get_bounds();
        let mut min = first_bounds.min;
        let mut max = first_bounds.max;
        
        for obj in &objects[1..] {
            let bounds = obj.get_bounds();
            min.x = min.x.min(bounds.min.x);
            min.y = min.y.min(bounds.min.y);
            min.z = min.z.min(bounds.min.z);
            max.x = max.x.max(bounds.max.x);
            max.y = max.y.max(bounds.max.y);
            max.z = max.z.max(bounds.max.z);
        }
        
        println!("📊 Bounds de la escena:");
        println!("   Min: ({:.1}, {:.1}, {:.1})", min.x, min.y, min.z);
        println!("   Max: ({:.1}, {:.1}, {:.1})", max.x, max.y, max.z);
        println!("   Centro aprox: ({:.1}, {:.1}, {:.1})", 
                 (min.x + max.x) / 2.0,
                 (min.y + max.y) / 2.0,
                 (min.z + max.z) / 2.0);
    }
    
    println!("📊 Tiempo total de carga: {:.3}s\n", start.elapsed().as_secs_f32());

    // Ajustar cámara según la escena elegida
    let mut camera = if scene_choice <= 3 {
        // Escenas de isla flotante - vista más cercana y centrada
        println!("📷 Posicionando cámara para vista de isla flotante...");
        Camera::new(
            Vector3::new(18.0, 0.0, 18.0),    // Más cerca, a nivel medio
            Vector3::new(0.0, -3.0, 0.0),     // Mirar ligeramente abajo del centro
            Vector3::new(0.0, 1.0, 0.0),
        )
    } else {
        // Otras escenas - cámara normal
        Camera::new(
            Vector3::new(15.0, 8.0, 15.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )
    };

    let rotation_speed = PI / 60.0;
    let zoom_speed = 0.5;

    println!("🎨 Renderizando escena inicial...");
    let render_start = std::time::Instant::now();
    render(&mut framebuffer, &bvh, &objects, &camera, &lights);
    println!("✨ Primera imagen renderizada en {:.3}s", render_start.elapsed().as_secs_f32());

    window.set_target_fps(30);

    println!("\n╔════════════════════════════════════════╗");
    println!("║            🎮 CONTROLES 🎮             ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  ← →  : Rotar horizontalmente          ║");
    println!("║  ↑ ↓  : Rotar verticalmente            ║");
    println!("║  W S  : Zoom in/out                    ║");
    println!("║  I    : Info de cámara                 ║");
    println!("║  R    : Reset cámara                   ║");
    println!("║  ESC  : Salir                          ║");
    println!("╚════════════════════════════════════════╝\n");

    if scene_choice <= 3 {
        println!("✨ Observa la isla flotante en el Overworld arriba");
        println!("🔥 y su reflejo oscuro en el Nether abajo!\n");
    }

    let mut frame_count = 0;
    let mut total_render_time = 0.0;

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
        
        // Info de cámara (tecla I)
        if window.is_key_pressed(KeyboardKey::KEY_I) {
            println!("\n📷 INFO DE CÁMARA:");
            println!("   Posición: ({:.1}, {:.1}, {:.1})", camera.eye.x, camera.eye.y, camera.eye.z);
            println!("   Mirando a: ({:.1}, {:.1}, {:.1})", camera.center.x, camera.center.y, camera.center.z);
            println!("   Distancia al centro: {:.1}", (camera.eye - camera.center).length());
        }
        
        // Reset cámara (tecla R)
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            if scene_choice <= 3 {
                camera = Camera::new(
                    Vector3::new(18.0, 0.0, 18.0),
                    Vector3::new(0.0, -3.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0),
                );
            } else {
                camera = Camera::new(
                    Vector3::new(15.0, 8.0, 15.0),
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0),
                );
            }
            println!("📷 Cámara reseteada");
            needs_render = true;
        }

        if needs_render {
            let frame_start = std::time::Instant::now();
            render(&mut framebuffer, &bvh, &objects, &camera, &lights);
            let elapsed = frame_start.elapsed().as_secs_f32();
            
            frame_count += 1;
            total_render_time += elapsed;
            let avg_time = total_render_time / frame_count as f32;
            
            println!("⚡ Frame {}: {:.3}s ({:.1} FPS) | Promedio: {:.3}s", 
                     frame_count, elapsed, 1.0 / elapsed, avg_time);
        }

        framebuffer.swap_buffers(&mut window, &thread);
    }

    println!("\n👋 ¡Gracias por usar el Ray Tracer!");
    println!("📊 Estadísticas finales:");
    println!("   Frames renderizados: {}", frame_count);
    if frame_count > 0 {
        println!("   Tiempo promedio: {:.3}s/frame", total_render_time / frame_count as f32);
    }
}