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
use scene_builder::{SceneBuilder, WallDirection};

const ORIGIN_BIAS: f32 = 1e-4;
const MAX_DEPTH: u32 = 2;

lazy_static::lazy_static! {
    static ref TEXTURE_MANAGER: Arc<TextureManager> = Arc::new(TextureManager::new());
}

/// Escena básica con islas flotantes y lagos orgánicos
fn create_floating_island_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let center_x = 0;
    let center_y = 12;
    let center_z = 0;
    let radius = 10;
    
    SceneBuilder::new()
        .use_obj_models(false)
        
        // Isla superior
        .add_floating_island(center_x, center_y, center_z, radius)
        .add_organic_lake(center_x - 2, center_z + 2, 2, 1)  // Lago pequeño
        .add_organic_lake(center_x + 3, center_z - 2, 3, 2)  // Lago grande
        .add_island_vegetation_auto(center_x, center_z, 0.08)
        
        // Reflejo Nether
        .add_nether_reflection(center_x, -center_y, center_z, radius)
        .add_nether_features(center_x, -center_y, center_z, radius)
        
        // Iluminación
        .add_dual_world_lighting(center_x as f32, center_z as f32)
        .build()
}

/// Escena con cascadas que caen desde el borde real de la isla
fn create_floating_island_with_waterfalls() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let center_x = 0;
    let center_y = 14;
    let center_z = 0;
    let radius = 8;
    
    let mut builder = SceneBuilder::new()
        .use_obj_models(true)
        
        // Generar isla
        .add_floating_island(center_x, center_y, center_z, radius)
        
        // Lagos en la cima (las cascadas vendrán de estos lagos)
        .add_organic_lake(center_x - 3, center_z, 2, 1)
        .add_organic_lake(center_x + 3, center_z, 2, 1)
        
        .add_island_vegetation_auto(center_x, center_z, 0.06);
    
    // Generar cascadas que caen desde posiciones lógicas
    let waterfall_positions = [
        (center_x - 5, center_z),      // Oeste
        (center_x + 5, center_z),      // Este
        (center_x, center_z - 5),      // Norte
        (center_x, center_z + 5),      // Sur
    ];
    
    for (wx, wz) in waterfall_positions {
        let top_y = center_y + radius;
        
        // Verificar que hay isla en esta posición
        if builder.is_position_occupied(wx, top_y - 1, wz) {
            // Cascada de agua desde la isla hacia abajo
            for h in 0..12 {
                let y = top_y - h;
                builder = builder.add_cube(
                    wx as f32, 
                    y as f32, 
                    wz as f32, 
                    0.5, 
                    "water"
                );
            }
        }
    }
    
    // Reflejo Nether con cascadas de lava invertidas
    builder = builder
        .add_nether_reflection(center_x, -center_y, center_z, radius)
        .add_nether_features(center_x, -center_y, center_z, radius);
    
    for (wx, wz) in waterfall_positions {
        let bottom_y = -center_y - radius;
        
        // Cascadas de lava hacia arriba
        for h in 0..12 {
            let y = bottom_y + h;
            builder = builder.add_cube(
                wx as f32, 
                y as f32, 
                wz as f32, 
                0.5, 
                "lava"
            );
        }
    }
    
    builder
        .add_dual_world_lighting(center_x as f32, center_z as f32)
        .build()
}

/// Escena con puente conectando dos mundos
fn create_floating_island_with_bridge() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let center_x = 0;
    let center_y = 12;
    let center_z = 0;
    let radius = 7;
    
    let mut builder = SceneBuilder::new()
        .use_obj_models(true)
        
        // Isla superior
        .add_floating_island(center_x, center_y, center_z, radius)
        .add_organic_lake(center_x, center_z, 2, 1)
        .add_island_vegetation_auto(center_x, center_z, 0.06)
        
        // Isla inferior (Nether)
        .add_nether_reflection(center_x, -center_y, center_z, radius)
        .add_nether_features(center_x, -center_y, center_z, radius);
    
    // Calcular puntos de conexión del puente
    let top_connection = center_y - radius - 1;
    let bottom_connection = -center_y + radius + 1;
    let bridge_length = top_connection - bottom_connection;
    
    println!("🌉 Construyendo puente de {} bloques", bridge_length);
    
    // Puente principal (pilares de vidrio)
    for y in bottom_connection..=top_connection {
        let progress = (y - bottom_connection) as f32 / bridge_length as f32;
        
        // Pilares laterales
        builder = builder
            .add_cube(center_x as f32 - 1.0, y as f32, center_z as f32, 0.4, "glass")
            .add_cube(center_x as f32 + 1.0, y as f32, center_z as f32, 0.4, "glass");
        
        // Piso del puente (cada 2 bloques)
        if y % 2 == 0 {
            builder = builder
                .add_cube(center_x as f32 - 0.5, y as f32, center_z as f32, 0.3, "stone")
                .add_cube(center_x as f32 + 0.5, y as f32, center_z as f32, 0.3, "stone");
        }
        
        // Luces en espiral (glowstone)
        if y % 4 == 0 {
            let angle = progress * std::f32::consts::PI * 4.0;
            let light_x = center_x as f32 + angle.cos() * 1.5;
            let light_z = center_z as f32 + angle.sin() * 1.5;
            
            builder = builder.add_cube(light_x, y as f32, light_z, 0.3, "glowstone");
        }
        
        // Refuerzos estructurales (cada 5 bloques)
        if y % 5 == 0 {
            builder = builder
                .add_cube(center_x as f32 - 1.0, y as f32, center_z as f32 - 1.0, 0.3, "stone")
                .add_cube(center_x as f32 + 1.0, y as f32, center_z as f32 - 1.0, 0.3, "stone")
                .add_cube(center_x as f32 - 1.0, y as f32, center_z as f32 + 1.0, 0.3, "stone")
                .add_cube(center_x as f32 + 1.0, y as f32, center_z as f32 + 1.0, 0.3, "stone");
        }
    }
    
    // Plataformas de entrada/salida
    for dx in -2..=2 {
        for dz in -2..=2 {
            // Plataforma superior
            builder = builder.add_cube(
                (center_x + dx) as f32,
                top_connection as f32,
                (center_z + dz) as f32,
                1.0,
                "stone"
            );
            
            // Plataforma inferior
            builder = builder.add_cube(
                (center_x + dx) as f32,
                bottom_connection as f32,
                (center_z + dz) as f32,
                1.0,
                "nether_brick"
            );
        }
    }
    
    builder
        .add_dual_world_lighting(center_x as f32, center_z as f32)
        .build()
}

/// Escena con múltiples islas pequeñas conectadas
fn create_archipelago_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut builder = SceneBuilder::new()
        .use_obj_models(true);
    
    // Isla central grande
    builder = builder
        .add_floating_island(0, 12, 0, 6)
        .add_organic_lake(0, 0, 2, 1)
        .add_island_vegetation_auto(0, 0, 0.1);
    
    // Islas satélite pequeñas
    let satellite_positions = [
        (12, 10, 8),
        (-10, 14, -6),
        (8, 11, -10),
        (-12, 13, 10),
    ];
    
    for (x, y, z) in satellite_positions {
        builder = builder
            .add_floating_island(x, y, z, 3)
            .add_island_vegetation_auto(x, z, 0.15);
    }
    
    // Reflejos Nether
    builder = builder
        .add_nether_reflection(0, -12, 0, 6);
    
    for (x, y, z) in satellite_positions {
        builder = builder
            .add_nether_reflection(x, -y, z, 3)
            .add_lava_lake(x, -y - 3, z, 2);
    }
    
    builder
        .add_dual_world_lighting(0.0, 0.0)
        .build()
}

fn create_castle_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .use_obj_models(true) // ⚡ ACTIVAR USO DE OBJ
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

fn create_house_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .use_obj_models(true) // ⚡ ACTIVAR USO DE OBJ
        .add_checkered_floor(10, "grass_top", "dirt")
        .add_house(0, 0)
        .add_tree(-5, 0,-5)
        .add_tree(-5,0 ,5)
        .add_tree(8, 0, -5)
        .add_tree(8, 0, 5)
        .add_sun(15.0, 20.0, 15.0, 3.5)
        .build()
}

fn create_simple_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .use_obj_models(true) // ⚡ ACTIVAR USO DE OBJ
        .add_floor(10, "grass_top")
        .add_cube(0.0, 1.0, 0.0, 2.0, "stone")
        .add_sun(10.0, 15.0, 10.0, 3.0)
        .build()
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
    
    println!("📦 ESCENAS DISPONIBLES:");
    println!("┌────────────────────────────────────────┐");
    println!("│ [1] 🏝️  Isla Flotante Básica          │");
    println!("│ [2] 💧 Isla con Cascadas               │");
    println!("│ [3] 🌉 Isla con Puente Portal          │");
    println!("│ [4] 🏰 Castillo Medieval               │");
    println!("│ [5] 🏠 Casa con Jardín                 │");
    println!("│ [6] 📦 Escena Simple                   │");
    println!("└────────────────────────────────────────┘\n");

    let mut scene_choice = 1;
    
    // Cargar escena inicial
    let (mut objects, mut lights) = load_scene(scene_choice);
    print_scene_info(scene_choice, objects.len(), lights.len());
    
    let mut camera = get_camera_for_scene(scene_choice);
    
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
    println!("║  1-6  : Cambiar escena (tiempo real)  ║");
    println!("║  ← →  : Rotar horizontalmente          ║");
    println!("║  ↑ ↓  : Rotar verticalmente            ║");
    println!("║  W S  : Zoom in/out                    ║");
    println!("║  R    : Reset cámara                   ║");
    println!("║  ESC  : Salir                          ║");
    println!("╚════════════════════════════════════════╝\n");

    if scene_choice <= 3 {
        println!("✨ TIP: Observa la isla flotante arriba");
        println!("🔥      y su reflejo del Nether abajo!\n");
    }

    println!("▶️  Programa iniciado. Presiona 1-6 para cambiar de escena.\n");

    let mut frame_count = 0;
    let mut total_render_time = 0.0;

    while !window.window_should_close() {
        let mut needs_render = false;
        let mut scene_changed = false;

        // Detectar cambio de escena
        let new_scene = if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            Some(1)
        } else if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            Some(2)
        } else if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            Some(3)
        } else if window.is_key_pressed(KeyboardKey::KEY_FOUR) {
            Some(4)
        } else if window.is_key_pressed(KeyboardKey::KEY_FIVE) {
            Some(5)
        } else if window.is_key_pressed(KeyboardKey::KEY_SIX) {
            Some(6)
        } else if window.is_key_pressed(KeyboardKey::KEY_T) {
            Some(6)
        } else {
            None
        };

        if let Some(new_scene_num) = new_scene {
            if new_scene_num != scene_choice {
                scene_choice = new_scene_num;
                scene_changed = true;
                
                println!("\n╔════════════════════════════════════════╗");
                println!("║     🔄 CAMBIANDO DE ESCENA...          ║");
                println!("╚════════════════════════════════════════╝\n");
                
                // Cargar nueva escena
                let start = std::time::Instant::now();
                (objects, lights) = load_scene(scene_choice);
                print_scene_info(scene_choice, objects.len(), lights.len());
                
                // Reconstruir BVH
                println!("⚡ Reconstruyendo BVH...");
                let bvh_start = std::time::Instant::now();
                bvh = BVH::build(&objects);
                println!("✅ BVH reconstruido en {:.3}s", bvh_start.elapsed().as_secs_f32());
                
                // Resetear cámara para la nueva escena
                camera = get_camera_for_scene(scene_choice);
                
                println!("⏱️  Tiempo total de cambio: {:.3}s\n", start.elapsed().as_secs_f32());
                
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
            camera = get_camera_for_scene(scene_choice);
            println!("📷 Cámara reseteada");
            needs_render = true;
        }

        if needs_render {
            let frame_start = std::time::Instant::now();
            render(&mut framebuffer, &bvh, &objects, &camera, &lights);
            let elapsed = frame_start.elapsed().as_secs_f32();
            
            frame_count += 1;
            total_render_time += elapsed;
            
            if frame_count % 30 == 0 && !scene_changed {
                let avg_time = total_render_time / frame_count as f32;
                println!("⚡ Frame {}: {:.3}s ({:.1} FPS avg)", 
                         frame_count, elapsed, 1.0 / avg_time);
            }
        }

        framebuffer.swap_buffers(&mut window, &thread);
    }

    println!("\n╔════════════════════════════════════════╗");
    println!("║         👋 PROGRAMA FINALIZADO         ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  📊 Estadísticas finales:              ║");
    println!("║     Frames renderizados: {}            ", frame_count);
    if frame_count > 0 {
        let avg = total_render_time / frame_count as f32;
        println!("║     Tiempo promedio: {:.3}s/frame      ", avg);
        println!("║     FPS promedio: {:.1}                ", 1.0 / avg);
    }
    println!("╚════════════════════════════════════════╝\n");
}

// Función auxiliar para cargar escenas
fn load_scene(scene_num: i32) -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    match scene_num {
        1 => create_floating_island_scene(),
        2 => create_floating_island_with_waterfalls(),
        3 => create_floating_island_with_bridge(),
        4 => create_castle_scene(),
        5 => create_house_scene(),
        6 => create_simple_scene(),
        _ => create_simple_scene(),
    }
}

// Función auxiliar para obtener cámara según escena
fn get_camera_for_scene(scene_num: i32) -> Camera {
    if scene_num <= 3 {
        Camera::new(
            Vector3::new(25.0, 0.0, 25.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )
    } else if scene_num == 4 {
        Camera::new(
            Vector3::new(25.0, 15.0, 25.0),
            Vector3::new(0.0, 5.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )
    } else {
        Camera::new(
            Vector3::new(15.0, 8.0, 15.0),
            Vector3::new(0.0, 2.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )
    }
}

// Función auxiliar para imprimir info de escena
fn print_scene_info(scene_num: i32, obj_count: usize, light_count: usize) {
    println!("╔════════════════════════════════════════╗");
    println!("║  🎬 ESCENA CARGADA: {}                  ║", scene_num);
    println!("╠════════════════════════════════════════╣");
    match scene_num {
        1 => println!("║  🏝️  Isla Flotante Básica             ║"),
        2 => println!("║  💧 Isla con Cascadas                  ║"),
        3 => println!("║  🌉 Isla con Puente Portal             ║"),
        4 => println!("║  🏰 Castillo Medieval                  ║"),
        5 => println!("║  🏠 Casa con Jardín                    ║"),
        6 => println!("║  📦 Escena Simple                      ║"),
        _ => println!("║  ❓ Escena Desconocida                 ║"),
    }
    println!("╠════════════════════════════════════════╣");
    println!("║  📊 Objetos: {:5}                      ║", obj_count);
    println!("║  💡 Luces: {:2}                          ║", light_count);
    println!("╚════════════════════════════════════════╝");
}