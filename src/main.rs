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
use ray_intersect::{Intersect, RayIntersect};
use camera::Camera;
use light::Light;
use material::{Material, vector3_to_color};
use cube::Cube;
use texture::TextureManager;
use mesh::Mesh;
use crate::scene_builder::{SceneBuilder, WallDirection};

// === EJEMPLO 1: ESCENA SIMPLE ===
fn create_simple_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .add_floor(10, "grass_top")              // Piso de césped 21x21
        .add_cube(0.0, 1.0, 0.0, 2.0, "stone")  // Cubo de piedra en el centro
        .add_sun(10.0, 15.0, 10.0, 3.0)         // Sol
        .build()
}

// === EJEMPLO 2: CASA CON JARDÍN ===
fn create_house_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        // Piso estilo tablero
        .add_checkered_floor(10, "grass_top", "dirt")
        
        // Casa principal
        .add_house(0, 0)
        
        // Árboles alrededor
        .add_tree(-5, -5)
        .add_tree(-5, 5)
        .add_tree(8, -5)
        .add_tree(8, 5)
        
        // Iluminación
        .add_sun(15.0, 20.0, 15.0, 3.5)
        .build()
}

// === EJEMPLO 3: CASTILLO ===
fn create_castle_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        // Piso de piedra
        .add_floor(20, "stone")
        
        // Torres en las esquinas
        .add_tower(-10, -10, 8, "stone")
        .add_tower(-10, 10, 8, "stone")
        .add_tower(10, -10, 8, "stone")
        .add_tower(10, 10, 8, "stone")
        
        // Murallas conectando las torres
        .add_wall(-10, -10, 21, 5, WallDirection::North, "stone")
        .add_wall(-10, 10, 21, 5, WallDirection::South, "stone")
        .add_wall(-10, -10, 21, 5, WallDirection::East, "stone")
        .add_wall(10, -10, 21, 5, WallDirection::West, "stone")
        
        // Torreones dorados en las torres
        .add_cube(-10.0, 8.0, -10.0, 1.0, "gold")
        .add_cube(-10.0, 8.0, 10.0, 1.0, "gold")
        .add_cube(10.0, 8.0, -10.0, 1.0, "gold")
        .add_cube(10.0, 8.0, 10.0, 1.0, "gold")
        
        // Antorchas en las murallas
        .add_torches(&[
            (-5.0, 5.0, -10.0),
            (0.0, 5.0, -10.0),
            (5.0, 5.0, -10.0),
            (-10.0, 5.0, -5.0),
            (-10.0, 5.0, 5.0),
        ])
        
        // Sol y luna (dos luces)
        .add_sun(20.0, 25.0, 20.0, 4.0)
        .add_light(-20.0, 15.0, -20.0, Color::new(150, 150, 200, 255), 2.0)
        
        .build()
}

// === EJEMPLO 4: GALERÍA DE MATERIALES ===
fn create_material_showcase() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let materials = vec![
        "stone", "wood", "gold", "silver", "glass",
        "water", "lava", "glowstone", "dirt", "leaves"
    ];
    
    let mut builder = SceneBuilder::new()
        .add_checkered_floor(10, "grass_top", "dirt");
    
    // Crear una fila de cubos con diferentes materiales
    for (i, material) in materials.iter().enumerate() {
        let x = (i as f32 - materials.len() as f32 / 2.0) * 2.5;
        builder = builder.add_cube(x, 1.0, 0.0, 1.5, material);
    }
    
    builder
        .add_sun(0.0, 20.0, 15.0, 4.0)
        .add_light(0.0, 5.0, -10.0, Color::new(255, 100, 100, 255), 2.0)
        .build()
}

// === EJEMPLO 5: PUEBLO ===
fn create_village_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        // Terreno
        .add_floor(25, "grass_top")
        
        // Casas distribuidas
        .add_house(-15, -15)
        .add_house(-15, 5)
        .add_house(5, -15)
        .add_house(5, 5)
        
        // Plaza central con fuente
        .add_cubes(
            &[
                (0.0, 0.0, 0.0),
                (1.0, 0.0, 0.0), (-1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0), (0.0, 0.0, -1.0),
            ],
            1.0,
            "stone"
        )
        .add_cube(0.0, 1.0, 0.0, 0.5, "water")
        
        // Árboles decorativos
        .add_tree(-20, 0)
        .add_tree(0, -20)
        .add_tree(20, 0)
        .add_tree(0, 20)
        .add_tree(-10, -10)
        .add_tree(10, 10)
        
        // Farolas
        .add_tower(-5, -5, 3, "stone")
        .add_torch(-5.0, 3.0, -5.0)
        .add_tower(5, 5, 3, "stone")
        .add_torch(5.0, 3.0, 5.0)
        
        // Iluminación ambiental
        .add_sun(30.0, 40.0, 30.0, 4.0)
        .add_light(-30.0, 20.0, -30.0, Color::new(200, 200, 255, 255), 1.5)
        
        .build()
}

// === EJEMPLO 6: LABERINTO ===
fn create_maze_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut builder = SceneBuilder::new()
        .add_floor(15, "stone");
    
    // Paredes del laberinto (patrón simple)
    let walls = vec![
        ((-10, -10), 5, WallDirection::North),
        ((-10, 0), 8, WallDirection::East),
        ((0, -5), 10, WallDirection::North),
        ((5, 0), 6, WallDirection::East),
    ];
    
    for ((x, z), length, direction) in walls {
        builder = builder.add_wall(x, z, length, 3, direction, "nether_brick");
    }
    
    builder
        .add_torch(0.0, 1.0, 0.0)
        .add_sun(20.0, 30.0, 20.0, 3.0)
        .build()
}

// === EJEMPLO 7: MODELO OBJ PERSONALIZADO ===
fn create_obj_showcase() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .add_checkered_floor(10, "grass_top", "dirt")
        
        // Cargar tu modelo OBJ
        .add_model("assets/cube.obj", 0.0, 2.0, 0.0, 2.0, "stone")
        .add_model("assets/cube.obj", -4.0, 1.0, 0.0, 1.5, "wood")
        .add_model("assets/cube.obj", 4.0, 1.0, 0.0, 1.5, "gold")
        
        // Pedestal para cada modelo
        .add_cube(0.0, 0.5, 0.0, 1.5, "stone")
        .add_cube(-4.0, 0.5, 0.0, 1.2, "dirt")
        .add_cube(4.0, 0.5, 0.0, 1.2, "dirt")
        
        .add_sun(10.0, 15.0, 10.0, 3.5)
        .add_light(-10.0, 10.0, -10.0, Color::new(100, 100, 255, 255), 2.0)
        .build()
}

// === EJEMPLO 8: ESCENA NETHER ===
fn create_nether_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        // Piso de netherrack
        .add_floor(15, "netherrack")
        
        // Estructuras de nether brick
        .add_box(-5, 0, -5, 10, 4, 10, "nether_brick")
        
        // Torres con soul sand
        .add_tower(-8, -8, 6, "soul_sand")
        .add_tower(8, -8, 6, "soul_sand")
        .add_tower(-8, 8, 6, "soul_sand")
        .add_tower(8, 8, 6, "soul_sand")
        
        // Lagos de lava
        .add_cubes(
            &[
                (12.0, 0.0, 0.0), (13.0, 0.0, 0.0),
                (12.0, 0.0, 1.0), (13.0, 0.0, 1.0),
            ],
            1.0,
            "lava"
        )
        
        // Iluminación rojiza
        .add_light(0.0, 10.0, 0.0, Color::new(255, 100, 50, 255), 3.0)
        .add_light(12.0, 1.0, 0.0, Color::new(255, 80, 20, 255), 2.5)
        .add_light(-10.0, 5.0, -10.0, Color::new(200, 50, 30, 255), 2.0)
        
        .build()
}

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
    let (objects, lights) = create_house_scene();

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