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
mod scenes;

use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect, BVH};
use camera::Camera;
use light::Light;
use material::{Material, vector3_to_color};
use texture::TextureManager;
use scenes::{SceneInfo, load_scene};

const ORIGIN_BIAS: f32 = 1e-4;
const MAX_DEPTH: u32 = 2;

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
    let fov = PI / 2.0;
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

fn print_scene_info(scene_num: i32, obj_count: usize, light_count: usize) {
    let info = SceneInfo::get(scene_num);
    println!("╔════════════════════════════════════════╗");
    println!("║  🎬 ESCENA CARGADA: {:2}                 ║", scene_num);
    println!("╠════════════════════════════════════════╣");
    println!("║  {:38} ║", info.name);
    println!("╠════════════════════════════════════════╣");
    println!("║  📊 Objetos: {:5}                      ║", obj_count);
    println!("║  💡 Luces: {:2}                          ║", light_count);
    println!("╚════════════════════════════════════════╝");
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("🏝️ Ray Tracer - Minecraft Style")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    println!("\n╔════════════════════════════════════════╗");
    println!("║   🏝️  RAY TRACER - MINECRAFT STYLE 🏝️  ║");
    println!("╚════════════════════════════════════════╝\n");
    
    println!("📦 ESCENAS DISPONIBLES:");
    println!("┌────────────────────────────────────────┐");
    println!("│ [1] 🏝️  Isla Flotante Básica          │");
    println!("│ [2] 💧 Isla con Cascadas               │");
    println!("│ [3] 🌉 Isla con Puente Portal          │");
    println!("│ [4] 🏰 Castillo Medieval               │");
    println!("│ [5] 🏠 Casa con Jardín                 │");
    println!("│ [6] 📦 Escena Simple                   │");
    println!("│ [7] 🏘️  Aldea Medieval                 │");
    println!("│ [8] 🌲 Bosque Encantado                │");
    println!("│ [9] 🏝️  Archipiélago Masivo            │");
    println!("│ [0] 🏛️  Templo Antiguo                 │");
    println!("│ [-] 🏔️  Cañón con Río                  │");
    println!("└────────────────────────────────────────┘\n");

    let mut scene_choice = 1;
    
    let (mut objects, mut lights) = load_scene(scene_choice);
    print_scene_info(scene_choice, objects.len(), lights.len());
    
    let scene_info = SceneInfo::get(scene_choice);
    let mut camera = Camera::new(
        scene_info.camera_pos,
        scene_info.camera_target,
        Vector3::new(0.0, 1.0, 0.0),
    );
    
    println!("⚡ Construyendo BVH...");
    let bvh_start = std::time::Instant::now();
    let mut bvh = BVH::build(&objects);
    println!("✅ BVH construido en {:.3}s\n", bvh_start.elapsed().as_secs_f32());

    let rotation_speed = PI / 60.0;
    let zoom_speed = 0.5;

    println!("🎨 Renderizando primera imagen...");
    let render_start = std::time::Instant::now();
    render(&mut framebuffer, &bvh, &objects, &camera, &lights);
    println!("✨ Renderizado inicial: {:.3}s\n", render_start.elapsed().as_secs_f32());

    window.set_target_fps(30);

    println!("╔════════════════════════════════════════╗");
    println!("║            🎮 CONTROLES 🎮             ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  1-9,0,- : Cambiar escena             ║");
    println!("║  ← →     : Rotar horizontalmente      ║");
    println!("║  ↑ ↓     : Rotar verticalmente        ║");
    println!("║  W S     : Zoom in/out                ║");
    println!("║  R       : Reset cámara               ║");
    println!("║  ESC     : Salir                      ║");
    println!("╚════════════════════════════════════════╝\n");

    let mut frame_count = 0;
    let mut total_render_time = 0.0;

    while !window.window_should_close() {
        let mut needs_render = false;

        // Detectar cambio de escena
        let new_scene = if window.is_key_pressed(KeyboardKey::KEY_ONE) { Some(1) }
            else if window.is_key_pressed(KeyboardKey::KEY_TWO) { Some(2) }
            else if window.is_key_pressed(KeyboardKey::KEY_THREE) { Some(3) }
            else if window.is_key_pressed(KeyboardKey::KEY_FOUR) { Some(4) }
            else if window.is_key_pressed(KeyboardKey::KEY_FIVE) { Some(5) }
            else if window.is_key_pressed(KeyboardKey::KEY_SIX) { Some(6) }
            else if window.is_key_pressed(KeyboardKey::KEY_SEVEN) { Some(7) }
            else if window.is_key_pressed(KeyboardKey::KEY_EIGHT) { Some(8) }
            else if window.is_key_pressed(KeyboardKey::KEY_NINE) { Some(9) }
            else if window.is_key_pressed(KeyboardKey::KEY_ZERO) { Some(10) }
            else if window.is_key_pressed(KeyboardKey::KEY_MINUS) { Some(11) }
            else { None };

        if let Some(new_scene_num) = new_scene {
            if new_scene_num != scene_choice {
                scene_choice = new_scene_num;
                
                println!("\n╔════════════════════════════════════════╗");
                println!("║     🔄 CAMBIANDO DE ESCENA...          ║");
                println!("╚════════════════════════════════════════╝\n");
                
                let start = std::time::Instant::now();
                (objects, lights) = load_scene(scene_choice);
                print_scene_info(scene_choice, objects.len(), lights.len());
                
                println!("⚡ Reconstruyendo BVH...");
                let bvh_start = std::time::Instant::now();
                bvh = BVH::build(&objects);
                println!("✅ BVH reconstruido en {:.3}s", bvh_start.elapsed().as_secs_f32());
                
                let scene_info = SceneInfo::get(scene_choice);
                camera = Camera::new(
                    scene_info.camera_pos,
                    scene_info.camera_target,
                    Vector3::new(0.0, 1.0, 0.0),
                );
                
                println!("⏱️  Tiempo total: {:.3}s\n", start.elapsed().as_secs_f32());
                
                needs_render = true;
                frame_count = 0;
                total_render_time = 0.0;
            }
        }

        // Controles de cámara
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
        
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            let scene_info = SceneInfo::get(scene_choice);
            camera = Camera::new(
                scene_info.camera_pos,
                scene_info.camera_target,
                Vector3::new(0.0, 1.0, 0.0),
            );
            println!("📷 Cámara reseteada");
            needs_render = true;
        }

        if needs_render {
            let frame_start = std::time::Instant::now();
            render(&mut framebuffer, &bvh, &objects, &camera, &lights);
            let elapsed = frame_start.elapsed().as_secs_f32();
            
            frame_count += 1;
            total_render_time += elapsed;
            
            if frame_count % 30 == 0 {
                let avg_time = total_render_time / frame_count as f32;
                println!("⚡ Frame {}: {:.3}s ({:.1} FPS)", 
                         frame_count, elapsed, 1.0 / avg_time);
            }
        }

        framebuffer.swap_buffers(&mut window, &thread);
    }

    println!("\n╔════════════════════════════════════════╗");
    println!("║         👋 PROGRAMA FINALIZADO         ║");
    println!("╚════════════════════════════════════════╝\n");
}