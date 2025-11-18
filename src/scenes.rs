use raylib::prelude::*;
use std::sync::Arc;
use crate::ray_intersect::RayIntersect;
use crate::light::Light;
use crate::scene_builder::{SceneBuilder, WallDirection};

/// ESCENA 1: Isla Flotante Básica
pub fn floating_island_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let center_x = 0;
    let center_y = 12;
    let center_z = 0;
    let radius = 10;
    
    SceneBuilder::new()
        .use_obj_models(false)
        .add_floating_island(center_x, center_y, center_z, radius)
        .add_organic_lake(center_x - 2, center_z + 2, 2, 1)
        .add_organic_lake(center_x + 3, center_z - 2, 3, 2)
        .add_island_vegetation_auto(center_x, center_z, 0.08)
        .add_nether_reflection(center_x, -center_y, center_z, radius)
        .add_nether_features(center_x, -center_y, center_z, radius)
        .add_dual_world_lighting(center_x as f32, center_z as f32)
        .build()
}

/// ESCENA 2: Isla con Cascadas
pub fn floating_island_waterfalls() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let center_x = 0;
    let center_y = 14;
    let center_z = 0;
    let radius = 8;
    
    let mut builder = SceneBuilder::new()
        .add_floating_island(center_x, center_y, center_z, radius)
        .add_organic_lake(center_x - 3, center_z, 2, 1)
        .add_organic_lake(center_x + 3, center_z, 2, 1)
        .add_island_vegetation_auto(center_x, center_z, 0.06);
    
    let waterfall_positions = [
        (center_x - 5, center_z),
        (center_x + 5, center_z),
        (center_x, center_z - 5),
        (center_x, center_z + 5),
    ];
    
    for (wx, wz) in waterfall_positions {
        let top_y = center_y + radius;
        
        if builder.is_position_occupied(wx, top_y - 1, wz) {
            // Cambiar a tamaño 1.0 para bloques sólidos conectados
            for h in 0..12 {
                let y = top_y - h;
                builder = builder.add_cube(wx as f32, y as f32, wz as f32, 1.0, "water");
            }
        }
    }
    
    builder = builder
        .add_nether_reflection(center_x, -center_y, center_z, radius)
        .add_nether_features(center_x, -center_y, center_z, radius);
    
    // Cascadas de lava en el Nether también con tamaño 1.0
    for (wx, wz) in waterfall_positions {
        let bottom_y = -center_y - radius;
        for h in 0..12 {
            let y = bottom_y + h;
            builder = builder.add_cube(wx as f32, y as f32, wz as f32, 1.0, "lava");
        }
    }
    
    builder
        .add_dual_world_lighting(center_x as f32, center_z as f32)
        .build()
}

/// ESCENA 3: Isla con Puente Portal
pub fn floating_island_bridge() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let center_x = 0;
    let center_y = 12;
    let center_z = 0;
    let radius = 7;
    
    let mut builder = SceneBuilder::new()
        .add_floating_island(center_x, center_y, center_z, radius)
        .add_organic_lake(center_x, center_z, 2, 1)
        .add_island_vegetation_auto(center_x, center_z, 0.06)
        .add_nether_reflection(center_x, -center_y, center_z, radius)
        .add_nether_features(center_x, -center_y, center_z, radius);
    
    let top_connection = center_y - radius - 1;
    let bottom_connection = -center_y + radius + 1;
    let bridge_length = top_connection - bottom_connection;
    
    println!("Construyendo puente de {} bloques", bridge_length);
    
    // Puente con bloques de tamaño 1.0
    for y in bottom_connection..=top_connection {
        let progress = (y - bottom_connection) as f32 / bridge_length as f32;
        
        // Columnas principales del puente (glass)
        builder = builder
            .add_cube((center_x - 1) as f32, y as f32, center_z as f32, 1.0, "glass")
            .add_cube((center_x + 1) as f32, y as f32, center_z as f32, 1.0, "glass");
        
        // Piso del puente (cada 2 bloques)
        if y % 2 == 0 {
            builder = builder
                .add_cube(center_x as f32, y as f32, center_z as f32, 1.0, "stone");
        }
        
        // Luces decorativas (cada 4 bloques)
        if y % 4 == 0 {
            let angle = progress * std::f32::consts::PI * 4.0;
            let light_x = center_x as f32 + angle.cos() * 2.0;
            let light_z = center_z as f32 + angle.sin() * 2.0;
            builder = builder.add_cube(
                light_x.round(), 
                y as f32, 
                light_z.round(), 
                1.0, 
                "glowstone"
            );
        }
        
        // Soportes diagonales (cada 5 bloques)
        if y % 5 == 0 {
            builder = builder
                .add_cube((center_x - 1) as f32, y as f32, (center_z - 1) as f32, 1.0, "stone")
                .add_cube((center_x + 1) as f32, y as f32, (center_z - 1) as f32, 1.0, "stone")
                .add_cube((center_x - 1) as f32, y as f32, (center_z + 1) as f32, 1.0, "stone")
                .add_cube((center_x + 1) as f32, y as f32, (center_z + 1) as f32, 1.0, "stone");
        }
    }
    
    // Plataformas de conexión con bloques de tamaño 1.0
    for dx in -2..=2 {
        for dz in -2..=2 {
            builder = builder
                .add_cube((center_x + dx) as f32, top_connection as f32, (center_z + dz) as f32, 1.0, "stone")
                .add_cube((center_x + dx) as f32, bottom_connection as f32, (center_z + dz) as f32, 1.0, "nether_brick");
        }
    }
    
    builder
        .add_dual_world_lighting(center_x as f32, center_z as f32)
        .build()
}

/// ESCENA 4: Castillo Medieval
pub fn castle_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
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

/// ESCENA 5: Casa con Jardín
pub fn house_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .add_checkered_floor(10, "grass_top", "dirt")
        .add_house(0, 0)
        .add_tree(-5, 0, -5)
        .add_tree(-5, 0, 5)
        .add_tree(8, 0, -5)
        .add_tree(8, 0, 5)
        .add_sun(15.0, 20.0, 15.0, 3.5)
        .build()
}

/// ESCENA 6: Escena Simple
pub fn simple_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    SceneBuilder::new()
        .add_floor(10, "grass_top")
        .add_cube(0.0, 1.0, 0.0, 2.0, "stone")
        .add_sun(10.0, 15.0, 10.0, 3.0)
        .build()
}

/// ESCENA 7: Aldea Medieval
pub fn village_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut builder = SceneBuilder::new();
    
    for x in -25_i32..=25 {
        for z in -25_i32..=25 {
            let is_road = (x.abs() < 3 && z.abs() < 20) || 
                         (z.abs() < 3 && x.abs() < 20);
            
            let material = if is_road {
                "stone"
            } else {
                if (x + z) % 3 == 0 { "grass_top" } else { "dirt" }
            };
            
            builder = builder.add_cube(x as f32, 0.0, z as f32, 1.0, material);
        }
    }
    
    let fountain_positions = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];
    for (x, z) in fountain_positions {
        builder = builder
            .add_cube(x as f32, 1.0, z as f32, 1.0, "stone")
            .add_cube(x as f32, 2.0, z as f32, 0.8, "water");
    }
    
    let house_positions = [
        (-15, -15), (-15, 8), (8, -15), (8, 8),
        (-15, -8), (8, -8), (-8, -15), (-8, 8),
    ];
    
    for (hx, hz) in house_positions {
        builder = builder.add_box(hx, 1, hz, 4, 3, 4, "wood");
        
        for level in 0..2 {
            let offset = level;
            for dx in offset..(4-offset) {
                for dz in offset..(4-offset) {
                    builder = builder.add_cube(
                        (hx + dx) as f32, 
                        (4 + level) as f32, 
                        (hz + dz) as f32, 
                        1.0, 
                        "stone"
                    );
                }
            }
        }
        
        builder = builder
            .add_cube((hx + 1) as f32, 4.0, (hz + 1) as f32, 0.5, "stone")
            .add_cube((hx + 1) as f32, 5.0, (hz + 1) as f32, 0.4, "stone")
            .add_torch((hx + 2) as f32, 2.0, hz as f32 - 0.5);
    }
    
    for i in 0..30 {
        let angle = (i as f32 / 30.0) * std::f32::consts::PI * 2.0;
        let radius = 18.0 + ((i * 7) % 5) as f32;
        let x = (angle.cos() * radius) as i32;
        let z = (angle.sin() * radius) as i32;
        
        if x.abs() > 5 || z.abs() > 5 {
            builder = builder.add_tree(x, 1, z);
        }
    }
    
    let tower_positions = [(-20, -20), (-20, 20), (20, -20), (20, 20)];
    for (tx, tz) in tower_positions {
        for y in 0..10 {
            builder = builder.add_cube(tx as f32, y as f32, tz as f32, 1.0, "stone");
        }
        builder = builder.add_torch(tx as f32, 10.0, tz as f32);
    }
    
    builder
        .add_sun(30.0, 40.0, 30.0, 4.0)
        .add_light(-20.0, 12.0, -20.0, Color::new(255, 200, 150, 255), 3.0)
        .add_light(20.0, 12.0, 20.0, Color::new(255, 200, 150, 255), 3.0)
        .build()
}

/// ESCENA 8: Bosque Encantado
pub fn enchanted_forest_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut builder = SceneBuilder::new();
    
    for x in -30..=30 {
        for z in -30..=30 {
            let wave = ((x as f32 * 0.2).sin() + (z as f32 * 0.15).cos()) * 2.0;
            let y = wave as i32;
            
            for dy in 0..=y.abs() {
                let actual_y = if y < 0 { -dy } else { dy };
                let material = if dy == y.abs() { "grass_top" } else { "dirt" };
                builder = builder.add_cube(x as f32, actual_y as f32, z as f32, 1.0, material);
            }
        }
    }
    
    for x in -28..=28 {
        for z in -28..=28 {
            let dist = ((x * x + z * z) as f32).sqrt();
            let noise = ((x as f32 * 0.3).sin() * (z as f32 * 0.4).cos()) * 10.0;
            let density = ((dist + noise) % 8.0) / 8.0;
            
            if density < 0.4 {
                let wave = ((x as f32 * 0.2).sin() + (z as f32 * 0.15).cos()) * 2.0;
                let y = wave as i32 + 1;
                builder = builder.add_tree(x, y, z);
            }
        }
    }
    
    let lake_positions = [(0, 0, 4), (-15, -10, 3), (12, 15, 3)];
    for (lx, lz, radius) in lake_positions {
        let wave = ((lx as f32 * 0.2).sin() + (lz as f32 * 0.15).cos()) * 2.0;
        let lake_y = wave as i32;
        
        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let dist = ((dx * dx + dz * dz) as f32).sqrt();
                let noise = ((dx as f32 * 0.5).sin() * (dz as f32 * 0.5).cos()) * 0.3;
                
                if dist < (radius as f32 * (0.9 + noise)) {
                    builder.remove_block(lx + dx, lake_y, lz + dz);
                    builder = builder.add_cube(
                        (lx + dx) as f32, 
                        lake_y as f32, 
                        (lz + dz) as f32, 
                        1.0, 
                        "water"
                    );
                }
            }
        }
    }
    
    let rock_positions: [(i32, i32); 3] = [(-20, 15), (18, -12), (-10, -20)];
    for (rx, rz) in rock_positions {
        let height = 5 + (((rx * rz).abs()) % 4) as i32;
        for y in 0..height {
            let size = (height - y) as f32 * 0.3 + 0.5;
            builder = builder.add_cube(rx as f32, y as f32, rz as f32, size, "stone");
        }
    }
    
    for i in 0..20 {
        let angle = (i as f32 / 20.0) * std::f32::consts::PI * 2.0;
        let radius = 10.0 + ((i * 3) % 8) as f32;
        let x = (angle.cos() * radius) as i32;
        let z = (angle.sin() * radius) as i32;
        let wave = ((x as f32 * 0.2).sin() + (z as f32 * 0.15).cos()) * 2.0;
        let y = wave as i32 + 1;
        
        builder = builder.add_cube(x as f32, y as f32, z as f32, 0.3, "glowstone");
    }
    
    builder
        .add_sun(40.0, 50.0, 40.0, 3.0)
        .add_light(0.0, 10.0, 0.0, Color::new(150, 255, 150, 255), 4.0)
        .add_light(-15.0, 8.0, -10.0, Color::new(100, 200, 255, 255), 3.0)
        .add_light(12.0, 8.0, 15.0, Color::new(255, 150, 200, 255), 3.0)
        .build()
}

/// ESCENA 9: Archipiélago Masivo
pub fn massive_archipelago_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut builder = SceneBuilder::new();
    
    builder = builder
        .add_floating_island(0, 15, 0, 8)
        .add_organic_lake(-2, 2, 2, 1)
        .add_island_vegetation_auto(0, 0, 0.06);
    
    for y in 0..12 {
        builder = builder.add_cube(0.0, (15 + 8 + y) as f32, 0.0, 1.0, "stone");
    }
    builder = builder.add_torch(0.0, (15 + 8 + 12) as f32, 0.0);
    
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0;
        let radius = 20.0;
        let x = (angle.cos() * radius) as i32;
        let z = (angle.sin() * radius) as i32;
        let y = 12 + ((i * 3) % 6) as i32 - 3;
        
        builder = builder
            .add_floating_island(x, y, z, 5)
            .add_island_vegetation_auto(x, z, 0.1);
        
        if i % 3 == 0 {
            builder = builder.add_organic_lake(x, z, 2, 1);
        }
        
        if i % 2 == 0 {
            for dy in 0..6 {
                builder = builder.add_cube(x as f32, (y + 5 + dy) as f32, z as f32, 0.8, "stone");
            }
            builder = builder.add_torch(x as f32, (y + 11) as f32, z as f32);
        }
    }
    
    for i in 0..16 {
        let angle = (i as f32 / 16.0) * std::f32::consts::PI * 2.0 + 0.2;
        let radius = 35.0;
        let x = (angle.cos() * radius) as i32;
        let z = (angle.sin() * radius) as i32;
        let y = 10 + ((i * 7) % 10) as i32 - 5;
        
        builder = builder
            .add_floating_island(x, y, z, 3)
            .add_island_vegetation_auto(x, z, 0.15);
    }
    
    for i in [0, 2, 4, 6] {
        let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0;
        let end_x = (angle.cos() * 20.0) as i32;
        let end_z = (angle.sin() * 20.0) as i32;
        let end_y = 12 + ((i * 3) % 6) as i32 - 3;
        
        let steps = 20;
        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let x = t * end_x as f32;
            let z = t * end_z as f32;
            let y = 15.0 + t * (end_y as f32 - 15.0) - (t * (1.0 - t) * 8.0);
            
            builder = builder
                .add_cube(x - 0.5, y, z, 0.3, "glass")
                .add_cube(x + 0.5, y, z, 0.3, "glass");
            
            if step % 5 == 0 {
                builder = builder.add_cube(x, y + 0.5, z, 0.2, "glowstone");
            }
        }
    }
    
    builder = builder
        .add_nether_reflection(0, -15, 0, 8)
        .add_nether_features(0, -15, 0, 8);
    
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0;
        let radius = 20.0;
        let x = (angle.cos() * radius) as i32;
        let z = (angle.sin() * radius) as i32;
        let y = -(12 + ((i * 3) % 6) as i32 - 3);
        
        builder = builder
            .add_nether_reflection(x, y, z, 5)
            .add_lava_lake(x, y - 5, z, 2);
    }
    
    builder
        .add_sun(0.0, 60.0, 40.0, 5.0)
        .add_light(0.0, 30.0, 0.0, Color::new(255, 255, 200, 255), 6.0)
        .add_light(-25.0, 20.0, -25.0, Color::new(150, 200, 255, 255), 3.0)
        .add_light(25.0, 20.0, 25.0, Color::new(255, 150, 150, 255), 3.0)
        .add_light(0.0, -30.0, 0.0, Color::new(255, 100, 50, 255), 5.0)
        .build()
}

/// ESCENA 10: Templo Antiguo
pub fn temple_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut builder = SceneBuilder::new();
    
    for x in -20..=20 {
        for z in -20..=20 {
            let dist = ((x * x + z * z) as f32).sqrt();
            if dist < 20.0 {
                builder = builder.add_cube(x as f32, 0.0, z as f32, 1.0, "stone");
            }
        }
    }
    
    for dir in 0..4 {
        let angle = (dir as f32 / 4.0) * std::f32::consts::PI * 2.0;

        for step in 0..8 {
            let radius = 15.0 - step as f32;

            let x = (angle.cos() * radius) as i32;
            let z = (angle.sin() * radius) as i32;
            
            for dx in -1..=1 {
                for dz in -1..=1 {
                    builder = builder.add_cube(
                        (x + dx) as f32, 
                        step as f32, 
                        (z + dz) as f32, 
                        1.0, 
                        "stone"
                    );
                }
            }
        }
    }
    for x in -8_i32..=8 {
        for z in -8_i32..=8 {
            if x.abs() == 8 || z.abs() == 8 {
                for y in 8..15 {
                    builder = builder.add_cube(x as f32, y as f32, z as f32, 1.0, "stone");
                }
            }
        }
    }
    
    let pillar_positions = [
        (-6, -6), (-6, 6), (6, -6), (6, 6),
        (-6, 0), (6, 0), (0, -6), (0, 6),
    ];
    
    for (px, pz) in pillar_positions {
        for y in 8..18 {
            builder = builder.add_cube(px as f32, y as f32, pz as f32, 1.0, "gold");
        }
        builder = builder.add_cube(px as f32, 18.0, pz as f32, 1.2, "gold");
    }
    
    for level in 0..6 {
        let size = 8 - level as i32;
        for x in -size..=size {
            for z in -size..=size {
                if x.abs() == size || z.abs() == size {
                    builder = builder.add_cube(
                        x as f32, 
                        (15 + level) as f32, 
                        z as f32, 
                        1.0, 
                        "gold"
                    );
                }
            }
        }
    }
    
    builder = builder.add_cube(0.0, 21.0, 0.0, 1.5, "glowstone");
    
    for i in 0..4 {
        let angle = (i as f32 / 4.0) * std::f32::consts::PI * 2.0 + std::f32::consts::PI / 4.0;
        let garden_x = (angle.cos() * 12.0) as i32;
        let garden_z = (angle.sin() * 12.0) as i32;
        
        for dx in -3..=3 {
            for dz in -3..=3 {
                builder = builder.add_cube(
                    (garden_x + dx) as f32, 
                    1.0, 
                    (garden_z + dz) as f32, 
                    1.0, 
                    "grass_top"
                );
            }
        }
        
        builder = builder
            .add_tree(garden_x - 2, 2, garden_z - 2)
            .add_tree(garden_x + 2, 2, garden_z - 2)
            .add_tree(garden_x - 2, 2, garden_z + 2)
            .add_tree(garden_x + 2, 2, garden_z + 2)
            .add_cube(garden_x as f32, 2.0, garden_z as f32, 0.8, "water");
    }
    
    for i in 0..16 {
        let angle = (i as f32 / 16.0) * std::f32::consts::PI * 2.0;
        let x = angle.cos() * 18.0;
        let z = angle.sin() * 18.0;
        builder = builder.add_torch(x, 1.0, z);
    }
    
    builder
        .add_sun(0.0, 80.0, 0.0, 6.0)
        .add_light(0.0, 22.0, 0.0, Color::new(255, 255, 150, 255), 8.0)
        .add_light(-12.0, 5.0, -12.0, Color::new(255, 200, 100, 255), 3.0)
        .add_light(12.0, 5.0, 12.0, Color::new(255, 200, 100, 255), 3.0)
        .build()
}

/// ESCENA 11: Cañón con Río
pub fn canyon_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut builder = SceneBuilder::new();
    
    for x in -30_i32..=30 {
        for z in -30_i32..=30 {
            let canyon_depth = if x.abs() < 8 {
                let depth_factor = 1.0 - (x.abs() as f32 / 8.0);
                (depth_factor * 10.0) as i32
            } else {
                0
            };
            
            let base_height = ((x as f32 * 0.1).sin() * (z as f32 * 0.08).cos() * 3.0) as i32;
            let height = base_height + 5 - canyon_depth;
            
            for y in 0..=height {
                let material = if y == height {
                    if canyon_depth > 0 { "stone" } else { "grass_top" }
                } else if y > height - 3 {
                    "dirt"
                } else {
                    "stone"
                };
                
                builder = builder.add_cube(x as f32, y as f32, z as f32, 1.0, material);
            }
        }
    }
    
    for z in -30_i32..=30 {
        for x in -3_i32..=3 {
            let depth = (1.0 - (x.abs() as f32 / 3.0) * 8.0) as i32;
            let y = -depth;
            
            builder.remove_block(x, y, z);
            builder = builder.add_cube(x as f32, y as f32, z as f32, 1.0, "water");
        }
    }
    
    for i in 0..20 {
        let side = if i % 2 == 0 { -1 } else { 1 };
        let z = -25 + (i * 3);
        let x = side * (10 + ((i * 7) % 5) as i32);
        
        let height = 3 + (i % 4);
        for y in 0..height {
            let width = height - y;
            for dx in 0..width {
                builder = builder.add_cube(
                    (x + dx * side) as f32, 
                    (5 + y) as f32, 
                    z as f32, 
                    1.0, 
                    "stone"
                );
            }
        }
    }
    
    for x in [-25, -20, -15, 15, 20, 25] {
        for z in (-25..=25).step_by(5) {
            let base_height = ((x as f32 * 0.1).sin() * (z as f32 * 0.08).cos() * 3.0) as i32 + 5;
            builder = builder.add_tree(x, base_height + 1, z);
        }
    }
    
    for bridge_z in [-15, 0, 15] {
        for dx in -7..=7 {
            let arch_height = ((dx as f32 / 7.0).powi(2) * 4.0) as i32;
            builder = builder.add_cube(
                dx as f32, 
                (8 + arch_height) as f32, 
                bridge_z as f32, 
                1.0, 
                "stone"
            );
        }
    }
    
    for side in [-1, 1] {
        let x = side * 7;
        for z in [-20, 0, 20] {
            for y in (0..8).rev() {
                builder = builder.add_cube(x as f32, y as f32, z as f32, 0.4, "water");
            }
        }
    }
    
    for i in 0..15 {
        let side = if i % 2 == 0 { -1 } else { 1 };
        let x = side * (8 + ((i * 3) % 4) as i32);
        let z = -20 + (i * 3);
        let y = 2 + (i % 3);
        
        builder = builder.add_cube(x as f32, y as f32, z as f32, 0.4, "glowstone");
    }
    
    builder
        .add_sun(0.0, 60.0, 50.0, 5.0)
        .add_light(0.0, 20.0, -20.0, Color::new(255, 220, 180, 255), 4.0)
        .add_light(0.0, 20.0, 20.0, Color::new(180, 220, 255, 255), 4.0)
        .add_light(0.0, -5.0, 0.0, Color::new(100, 150, 255, 255), 3.0)
        .build()
}

/// ESCENA 12: Portal Dimensional
pub fn portal_scene() -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    let mut builder = SceneBuilder::new();
    
    // Plataforma base de obsidiana
    for x in -15..=15 {
        for z in -15..=15 {
            builder = builder.add_cube(x as f32, 0.0, z as f32, 1.0, "obsidian");
        }
    }
    
    // Marco del portal (obsidiana)
    let portal_height = 10;
    
    // Columnas laterales
    for y in 1..=portal_height {
        builder = builder
            .add_cube(-3.0, y as f32, 0.0, 1.0, "obsidian")
            .add_cube(3.0, y as f32, 0.0, 1.0, "obsidian");
    }
    
    // Techo
    for x in -2..=2 {
        builder = builder.add_cube(x as f32, (portal_height + 1) as f32, 0.0, 1.0, "obsidian");
    }
    
    // Interior del portal (material portal con efectos especiales)
    for y in 2..=portal_height {
        for x in -1..=1 {
            builder = builder.add_cube(x as f32, y as f32, 0.0, 1.0, "portal");
        }
    }
    
    // Decoración: Bloques de glowstone alrededor
    let glow_positions = [
        (-5, 1, -3), (-5, 1, 3), (5, 1, -3), (5, 1, 3),
        (-5, 5, -3), (-5, 5, 3), (5, 5, -3), (5, 5, 3),
    ];
    
    for (x, y, z) in glow_positions {
        builder = builder.add_cube(x as f32, y as f32, z as f32, 1.0, "glowstone");
    }
    
    // Camino de obsidiana
    for z in 5..15 {
        for x in -1..=1 {
            builder = builder.add_cube(x as f32, 1.0, z as f32, 1.0, "obsidian");
        }
    }
    
    // Pilares decorativos con materiales refractivos
    let pillar_data = [
        (-8, -8, "diamond"),
        (-8, 8, "emerald"),
        (8, -8, "emerald"),
        (8, 8, "diamond"),
    ];
    
    for (px, pz, mat) in pillar_data {
        for y in 1..6 {
            builder = builder.add_cube(px as f32, y as f32, pz as f32, 1.0, mat);
        }
        // Corona de hielo en la cima
        for dx in -1_i32..=1 {
            for dz in -1_i32..=1 {
                if dx.abs() + dz.abs() == 1 {
                    builder = builder.add_cube((px + dx) as f32, 6.0, (pz + dz) as f32, 1.0, "ice");
                }
            }
        }
    }
    
    // Árboles cristalizados
    for (tx, tz) in [(-12, -10), (12, -10), (-10, 12), (10, 12)] {
        // Tronco de hielo
        for y in 1..5 {
            builder = builder.add_cube(tx as f32, y as f32, tz as f32, 1.0, "ice");
        }
        // Copa de esmeralda
        for dx in -1..=1 {
            for dz in -1..=1 {
                if dx != 0 || dz != 0 {
                    builder = builder.add_cube(
                        (tx + dx) as f32,
                        5.0,
                        (tz + dz) as f32,
                        1.0,
                        "emerald"
                    );
                }
            }
        }
    }
    
    // Piscina de agua refractiva con bordes de diamante
    for x in -3_i32..=3 {
        for z in 8_i32..=12 {
            if x.abs() == 3 || z == 8 || z == 12 {
                builder = builder.add_cube(x as f32, 1.0, z as f32, 1.0, "diamond");
            } else {
                builder = builder.add_cube(x as f32, 1.0, z as f32, 1.0, "water");
            }
        }
    }
    
    // Anillos de glowstone flotantes alrededor del portal
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0;
        let radius = 7.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        
        builder = builder.add_cube(x, 6.0, z, 0.5, "glowstone");
    }
    
    // Círculo de portales pequeños decorativos
    for i in 0..12 {
        let angle = (i as f32 / 12.0) * std::f32::consts::PI * 2.0;
        let radius = 13.0;
        let x = (angle.cos() * radius) as i32;
        let z = (angle.sin() * radius) as i32;
        
        for y in 1..3 {
            builder = builder.add_cube(x as f32, y as f32, z as f32, 0.6, "portal");
        }
    }
    
    builder
        .add_sun(0.0, 30.0, 20.0, 5.0)
        .add_light(0.0, 6.0, 0.0, Color::new(200, 100, 255, 255), 8.0) // Luz del portal
        .add_light(-8.0, 4.0, -8.0, Color::new(100, 200, 255, 255), 3.0)
        .add_light(8.0, 4.0, 8.0, Color::new(100, 255, 200, 255), 3.0)
        .add_light(0.0, 10.0, 10.0, Color::new(150, 200, 255, 255), 2.5)
        .build()
}

/// Configuración de escenas
pub struct SceneInfo {
    pub name: &'static str,
    pub camera_pos: Vector3,
    pub camera_target: Vector3,
}

impl SceneInfo {
    pub fn get(scene_num: i32) -> Self {
        match scene_num {
            1 => SceneInfo {
                name: "Isla Flotante Básica",
                camera_pos: Vector3::new(25.0, 0.0, 25.0),
                camera_target: Vector3::new(0.0, 0.0, 0.0),
            },
            2 => SceneInfo {
                name: "Isla con Cascadas",
                camera_pos: Vector3::new(25.0, 0.0, 25.0),
                camera_target: Vector3::new(0.0, 0.0, 0.0),
            },
            3 => SceneInfo {
                name: "Isla con Puente Portal",
                camera_pos: Vector3::new(25.0, 0.0, 25.0),
                camera_target: Vector3::new(0.0, 0.0, 0.0),
            },
            4 => SceneInfo {
                name: "Castillo Medieval",
                camera_pos: Vector3::new(25.0, 15.0, 25.0),
                camera_target: Vector3::new(0.0, 5.0, 0.0),
            },
            5 => SceneInfo {
                name: "Casa con Jardín",
                camera_pos: Vector3::new(15.0, 8.0, 15.0),
                camera_target: Vector3::new(0.0, 2.0, 0.0),
            },
            6 => SceneInfo {
                name: "Escena Simple",
                camera_pos: Vector3::new(15.0, 8.0, 15.0),
                camera_target: Vector3::new(0.0, 2.0, 0.0),
            },
            7 => SceneInfo {
                name: "Aldea Medieval",
                camera_pos: Vector3::new(35.0, 20.0, 35.0),
                camera_target: Vector3::new(0.0, 3.0, 0.0),
            },
            8 => SceneInfo {
                name: "Bosque Encantado",
                camera_pos: Vector3::new(40.0, 15.0, 40.0),
                camera_target: Vector3::new(0.0, 5.0, 0.0),
            },
            9 => SceneInfo {
                name: "Archipiélago Masivo",
                camera_pos: Vector3::new(50.0, 20.0, 50.0),
                camera_target: Vector3::new(0.0, 10.0, 0.0),
            },
            10 => SceneInfo {
                name: "Templo Antiguo",
                camera_pos: Vector3::new(30.0, 25.0, 30.0),
                camera_target: Vector3::new(0.0, 10.0, 0.0),
            },
            11 => SceneInfo {
                name: "Cañón con Río",
                camera_pos: Vector3::new(0.0, 25.0, 40.0),
                camera_target: Vector3::new(0.0, 5.0, 0.0),
            },
            12 => SceneInfo {
                name: "Portal Dimensional",
                camera_pos: Vector3::new(0.0, 8.0, 25.0),
                camera_target: Vector3::new(0.0, 5.0, 0.0),
            },            
            _ => SceneInfo {
                name: "Escena Desconocida",
                camera_pos: Vector3::new(15.0, 8.0, 15.0),
                camera_target: Vector3::new(0.0, 2.0, 0.0),
            },
        }
    }
}

/// Carga una escena según su número
pub fn load_scene(scene_num: i32) -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
    match scene_num {
        1 => floating_island_scene(),
        2 => floating_island_waterfalls(),
        3 => floating_island_bridge(),
        4 => castle_scene(),
        5 => house_scene(),
        6 => simple_scene(),
        7 => village_scene(),
        8 => enchanted_forest_scene(),
        9 => massive_archipelago_scene(),
        10 => temple_scene(),
        11 => canyon_scene(),
        12 => portal_scene(),
        _ => simple_scene(),
    }
}