use raylib::prelude::*;
use std::sync::Arc;
use std::collections::HashSet;
use crate::ray_intersect::RayIntersect;
use crate::light::Light;
use crate::material::Material;
use crate::cube::Cube;
use crate::mesh::Mesh;

pub struct SceneBuilder {
    objects: Vec<Arc<dyn RayIntersect + Send + Sync>>,
    lights: Vec<Light>,
    materials: MaterialLibrary,
    use_obj_cubes: bool,
    grass_positions: Vec<(i32, i32, i32)>,
    occupied_positions: HashSet<(i32, i32, i32)>,
}
struct MaterialLibrary {
    materials: std::collections::HashMap<String, Material>,
}

impl MaterialLibrary {
    fn new() -> Self {
        let mut materials = std::collections::HashMap::new();
        
        materials.insert("grass_top".to_string(), Self::grass_top());
        materials.insert("grass_side".to_string(), Self::grass_side());
        materials.insert("dirt".to_string(), Self::dirt());
        materials.insert("stone".to_string(), Self::stone());
        materials.insert("wood".to_string(), Self::wood());
        materials.insert("leaves".to_string(), Self::leaves());
        materials.insert("water".to_string(), Self::water());
        materials.insert("lava".to_string(), Self::lava());
        materials.insert("glowstone".to_string(), Self::glowstone());
        materials.insert("glass".to_string(), Self::glass());
        materials.insert("mirror".to_string(), Self::mirror());
        materials.insert("gold".to_string(), Self::gold());
        materials.insert("silver".to_string(), Self::silver());
        materials.insert("netherrack".to_string(), Self::netherrack());
        materials.insert("nether_brick".to_string(), Self::nether_brick());
        materials.insert("soul_sand".to_string(), Self::soul_sand());
        
        MaterialLibrary { materials }
    }
    
    fn get(&self, name: &str) -> Material {
        self.materials.get(name).cloned().unwrap_or_else(|| Self::stone())
    }
    
    fn grass_top() -> Material {
        Material::new(Vector3::new(0.2, 0.8, 0.2), 10.0, [0.9, 0.1], 0.0, 0.0, Vector3::zero(), Some("grass_top".to_string()))
    }
    fn grass_side() -> Material {
        Material::new(Vector3::new(0.2, 0.8, 0.2), 10.0, [0.9, 0.1], 0.0, 0.0, Vector3::zero(), Some("grass_side".to_string()))
    }
    fn dirt() -> Material {
        Material::new(Vector3::new(0.6, 0.4, 0.2), 5.0, [0.85, 0.05], 0.0, 0.0, Vector3::zero(), Some("dirt".to_string()))
    }
    fn stone() -> Material {
        Material::new(Vector3::new(0.5, 0.5, 0.5), 15.0, [0.8, 0.15], 0.0, 0.0, Vector3::zero(), Some("stone".to_string()))
    }
    fn wood() -> Material {
        Material::new(Vector3::new(0.4, 0.25, 0.1), 8.0, [0.75, 0.08], 0.0, 0.0, Vector3::zero(), Some("wood".to_string()))
    }
    fn leaves() -> Material {
        Material::new(Vector3::new(0.1, 0.5, 0.1), 5.0, [0.85, 0.05], 0.0, 0.0, Vector3::zero(), Some("leaves".to_string()))
    }
    fn water() -> Material {
        Material::new(Vector3::new(0.1, 0.3, 0.7), 100.0, [0.2, 0.6], 0.0, 0.5, Vector3::zero(), Some("water".to_string()))
    }
    fn lava() -> Material {
        Material::new(Vector3::new(1.0, 0.3, 0.0), 20.0, [0.6, 0.3], 0.0, 0.0, Vector3::new(1.0, 0.4, 0.05), Some("lava".to_string()))
    }
    fn glowstone() -> Material {
        Material::new(Vector3::new(1.0, 0.9, 0.6), 30.0, [0.5, 0.3], 0.0, 0.0, Vector3::new(1.5, 1.2, 0.6), Some("glowstone".to_string()))
    }
    fn glass() -> Material {
        Material::new(Vector3::new(0.9, 0.9, 1.0), 150.0, [0.1, 0.8], 0.7, 0.4, Vector3::zero(), None)
    }
    fn mirror() -> Material {
        Material::new(Vector3::new(0.9, 0.9, 1.0), 120.0, [0.05, 0.6], 0.0, 0.85, Vector3::zero(), None)
    }
    fn gold() -> Material {
        Material::new(Vector3::new(1.0, 0.85, 0.0), 200.0, [0.2, 0.7], 0.0, 0.9, Vector3::zero(), None)
    }
    fn silver() -> Material {
        Material::new(Vector3::new(0.95, 0.95, 0.95), 180.0, [0.15, 0.75], 0.0, 0.95, Vector3::zero(), None)
    }
    fn netherrack() -> Material {
        Material::new(Vector3::new(0.6, 0.1, 0.1), 10.0, [0.8, 0.1], 0.0, 0.0, Vector3::zero(), Some("netherrack".to_string()))
    }
    fn nether_brick() -> Material {
        Material::new(Vector3::new(0.2, 0.05, 0.05), 15.0, [0.7, 0.15], 0.0, 0.0, Vector3::zero(), Some("nether_brick".to_string()))
    }
    fn soul_sand() -> Material {
        Material::new(Vector3::new(0.3, 0.2, 0.15), 8.0, [0.75, 0.1], 0.0, 0.0, Vector3::zero(), Some("soul_sand".to_string()))
    }
}


impl SceneBuilder {
    pub fn new() -> Self {
        SceneBuilder {
            objects: Vec::new(),
            lights: Vec::new(),
            materials: MaterialLibrary::new(),
            use_obj_cubes: false,
            grass_positions: Vec::new(),
            occupied_positions: HashSet::new(),
        }
    }
    
    pub fn use_obj_models(mut self, use_obj: bool) -> Self {
        self.use_obj_cubes = use_obj;
        self
    }

    pub fn is_position_occupied(&self, x: i32, y: i32, z: i32) -> bool {
        self.occupied_positions.contains(&(x, y, z))
    }

    fn mark_position(&mut self, x: i32, y: i32, z: i32) {
        self.occupied_positions.insert((x, y, z));
    }

    pub fn remove_block(&mut self, x: i32, y: i32, z: i32) {
        self.occupied_positions.remove(&(x, y, z));
    }
    
    pub fn add_cube(mut self, x: f32, y: f32, z: f32, size: f32, material: &str) -> Self {
        let xi = x as i32;
        let yi = y as i32;
        let zi = z as i32;
        
        // Verificar si ya existe un bloque aquí
        if self.is_position_occupied(xi, yi, zi) {
            return self;
        }
        
        let mat = self.materials.get(material);
        
        if self.use_obj_cubes {
            match Mesh::from_obj("assets/cube.obj", &mat, Vector3::new(x, y, z), size) {
                Ok(mesh) => {
                    self.objects.extend(mesh.to_objects());
                }
                Err(e) => {
                    println!("  Error cargando cube.obj: {}. Usando cubo procedural.", e);
                    self.objects.push(Arc::new(Cube::new(Vector3::new(x, y, z), size, mat)));
                }
            }
        } else {
            self.objects.push(Arc::new(Cube::new(Vector3::new(x, y, z), size, mat)));
        }
        
        // Marcar posición como ocupada
        self.mark_position(xi, yi, zi);
        
        self
    }

    pub fn add_floor(mut self, radius: i32, material: &str) -> Self {
        let mat = self.materials.get(material);
        for x in -radius..=radius {
            for z in -radius..=radius {
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new(x as f32, 0.0, z as f32), 
                    1.0, 
                    mat.clone()
                )));
            }
        }
        self
    }
    
    pub fn add_checkered_floor(mut self, radius: i32, material1: &str, material2: &str) -> Self {
        let mat1 = self.materials.get(material1);
        let mat2 = self.materials.get(material2);
        
        for x in -radius..=radius {
            for z in -radius..=radius {
                let mat = if (x + z) % 2 == 0 { mat1.clone() } else { mat2.clone() };
                self.objects.push(Arc::new(Cube::new(Vector3::new(x as f32, 0.0, z as f32), 1.0, mat)));
            }
        }
        self
    }
    
    pub fn add_wall(mut self, start_x: i32, start_z: i32, length: i32, height: i32, direction: WallDirection, material: &str) -> Self {
        let mat = self.materials.get(material);
        
        for y in 0..height {
            for i in 0..length {
                let (x, z) = match direction {
                    WallDirection::North | WallDirection::South => (start_x + i, start_z),
                    WallDirection::East | WallDirection::West => (start_x, start_z + i),
                };
                
                self.objects.push(Arc::new(Cube::new(Vector3::new(x as f32, y as f32, z as f32), 1.0, mat.clone())));
            }
        }
        self
    }
    
    pub fn add_tower(mut self, x: i32, z: i32, height: i32, material: &str) -> Self {
        let mat = self.materials.get(material);
        for y in 0..height {
            self.objects.push(Arc::new(Cube::new(Vector3::new(x as f32, y as f32, z as f32), 1.0, mat.clone())));
        }
        self
    }
    
    pub fn add_box(mut self, x: i32, y: i32, z: i32, width: i32, height: i32, depth: i32, material: &str) -> Self {
        let mat = self.materials.get(material);
        
        for dy in 0..height {
            for dx in 0..width {
                self.objects.push(Arc::new(Cube::new(Vector3::new((x + dx) as f32, (y + dy) as f32, z as f32), 1.0, mat.clone())));
                self.objects.push(Arc::new(Cube::new(Vector3::new((x + dx) as f32, (y + dy) as f32, (z + depth - 1) as f32), 1.0, mat.clone())));
            }
            
            for dz in 1..depth-1 {
                self.objects.push(Arc::new(Cube::new(Vector3::new(x as f32, (y + dy) as f32, (z + dz) as f32), 1.0, mat.clone())));
                self.objects.push(Arc::new(Cube::new(Vector3::new((x + width - 1) as f32, (y + dy) as f32, (z + dz) as f32), 1.0, mat.clone())));
            }
        }
        
        for dx in 0..width {
            for dz in 0..depth {
                self.objects.push(Arc::new(Cube::new(Vector3::new((x + dx) as f32, y as f32, (z + dz) as f32), 1.0, mat.clone())));
                self.objects.push(Arc::new(Cube::new(Vector3::new((x + dx) as f32, (y + height - 1) as f32, (z + dz) as f32), 1.0, mat.clone())));
            }
        }
        
        self
    }
    
    pub fn add_model(mut self, path: &str, x: f32, y: f32, z: f32, scale: f32, material: &str) -> Self {
        let mat = self.materials.get(material);
        
        match Mesh::from_obj(path, &mat, Vector3::new(x, y, z), scale) {
            Ok(mesh) => {
                self.objects.extend(mesh.to_objects());
            }
            Err(_) => {
                self.objects.push(Arc::new(Cube::new(Vector3::new(x, y, z), scale, mat)));
            }
        }
        self
    }
    
    pub fn add_light(mut self, x: f32, y: f32, z: f32, color: Color, intensity: f32) -> Self {
        self.lights.push(Light::new(Vector3::new(x, y, z), color, intensity));
        self
    }
    
    pub fn add_sun(self, x: f32, y: f32, z: f32, intensity: f32) -> Self {
        self.add_light(x, y, z, Color::new(255, 250, 240, 255), intensity)
    }
    
    pub fn add_torch(mut self, x: f32, y: f32, z: f32) -> Self {
        let torch_mat = Material::new(Vector3::new(1.0, 0.6, 0.0), 40.0, [0.3, 0.2], 0.0, 0.0, Vector3::new(1.2, 0.6, 0.1), Some("glowstone".to_string()));
        
        self.objects.push(Arc::new(Cube::new(Vector3::new(x, y, z), 0.3, torch_mat)));
        self.lights.push(Light::new(Vector3::new(x, y + 0.5, z), Color::new(255, 180, 80, 255), 2.5));
        
        self
    }
    
    pub fn add_torches(mut self, positions: &[(f32, f32, f32)]) -> Self {
        for (x, y, z) in positions {
            self = self.add_torch(*x, *y, *z);
        }
        self
    }
    
    pub fn add_tree(mut self, x: i32, y: i32, z: i32) -> Self {
        // Tronco del árbol (5 bloques de altura desde Y)
        for dy in 0..5 {
            self = self.add_cube(x as f32, (y + dy) as f32, z as f32, 1.0, "wood");
        }
        
        // Hojas (relativas a Y)
        let leaf_positions = [
            (x, y + 4, z-1), (x, y + 4, z+1), (x-1, y + 4, z), (x+1, y + 4, z),
            (x, y + 5, z-1), (x, y + 5, z+1), (x-1, y + 5, z), (x+1, y + 5, z),
            (x, y + 5, z), (x, y + 6, z),
        ];
        
        for (lx, ly, lz) in leaf_positions {
            self = self.add_cube(lx as f32, ly as f32, lz as f32, 1.0, "leaves");
        }
        
        self
    }
    
    pub fn add_house(mut self, x: i32, z: i32) -> Self {
        self = self.add_box(x, 0, z, 5, 4, 5, "wood");
        
        for level in 0..3 {
            let offset = level;
            for dx in offset..(5-offset) {
                for dz in offset..(5-offset) {
                    self = self.add_cube((x + dx) as f32, (4 + level) as f32, (z + dz) as f32, 1.0, "stone");
                }
            }
        }
        
        self
    }
    
    pub fn add_floating_island(mut self, center_x: i32, center_y: i32, center_z: i32, radius: i32) -> Self {
        let cx = center_x as f32;
        let cy = center_y as f32;
        let cz = center_z as f32;
        let r = radius as f32;
        
        self.grass_positions.clear();
        
        for y in -radius..=radius {
            for x in -radius..=radius {
                for z in -radius..=radius {
                    let fx = x as f32;
                    let fy = y as f32;
                    let fz = z as f32;
                    
                    let dist = ((fx * fx + fy * fy * 1.5 + fz * fz).sqrt()) / r;
                    let density = (1.0 - dist) + (fy / r) * 0.3;
                    let noise = ((fx * 0.3 + fz * 0.5).sin() * (fy * 0.4).cos()) * 0.2;
                    
                    if density + noise > 0.2 {
                        let wx = cx + fx;
                        let wy = cy + fy;
                        let wz = cz + fz;
                        
                        let material = if fy > radius as f32 * 0.5 {
                            self.grass_positions.push((
                                wx as i32,
                                wy as i32,
                                wz as i32
                            ));
                            "grass_top"
                        } else if fy > radius as f32 * 0.0 {
                            "dirt"
                        } else {
                            "stone"
                        };
                        
                        self = self.add_cube(wx, wy, wz, 1.0, material);
                    }
                }
            }
        }
        
        self
    }

    /// Genera un lago orgánico en la superficie de la isla
    pub fn add_organic_lake(
        mut self,
        center_x: i32,
        center_z: i32,
        radius: i32,
        depth: i32,
    ) -> Self {
        use std::collections::HashMap;
        
        // 1. Encontrar superficie (bloques de grass más altos)
        let mut surface_map: HashMap<(i32, i32), i32> = HashMap::new();
        for (x, y, z) in &self.grass_positions {
            surface_map
                .entry((*x, *z))
                .and_modify(|max_y| *max_y = (*max_y).max(*y))
                .or_insert(*y);
        }
        
        // 2. Generar forma orgánica del lago
        let mut lake_positions = Vec::new();
        
        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let x = center_x + dx;
                let z = center_z + dz;
                
                // Verificar si hay superficie aquí
                if let Some(&surface_y) = surface_map.get(&(x, z)) {
                    // Usar múltiples octavas de ruido para forma orgánica
                    let noise1 = ((dx as f32 * 0.3).sin() * (dz as f32 * 0.3).cos()) * 0.5;
                    let noise2 = ((dx as f32 * 0.7 + dz as f32 * 0.5).sin()) * 0.3;
                    let noise3 = ((dx as f32 * 1.2).cos() * (dz as f32 * 0.9).sin()) * 0.2;
                    
                    let dist = ((dx * dx + dz * dz) as f32).sqrt();
                    let threshold = radius as f32 * (0.8 + noise1 + noise2 + noise3);
                    
                    if dist < threshold {
                        lake_positions.push((x, surface_y, z));
                    }
                }
            }
        }
        
        let lake_size = lake_positions.len(); // Guardar tamaño antes del loop
        
        // 3. Excavar el lago y llenar con agua
        for &(x, surface_y, z) in &lake_positions {  // ✅ Usar referencia
            // Remover bloques de tierra/grass
            for dy in 0..=depth {
                let y = surface_y - dy;
                
                // Remover de occupied_positions para poder reemplazar
                self.occupied_positions.remove(&(x, y, z));
                
                // Agua en el fondo, aire arriba
                if dy == depth {
                    self = self.add_cube(x as f32, y as f32, z as f32, 1.0, "water");
                }
                // Los otros bloques simplemente se remueven (quedan como aire)
            }
        }
        
        println!("💧 Lago generado con {} bloques de agua", lake_size);
        
        self
    }
    
    /// Genera un lago de lava en el Nether
    pub fn add_lava_lake(
        mut self,
        center_x: i32,
        center_y: i32,
        center_z: i32,
        radius: i32,
    ) -> Self {
        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let x = center_x + dx;
                let z = center_z + dz;
                
                // Forma orgánica
                let noise = ((dx as f32 * 0.4).sin() * (dz as f32 * 0.4).cos()) * 0.4;
                let dist = ((dx * dx + dz * dz) as f32).sqrt();
                let threshold = radius as f32 * (0.85 + noise);
                
                if dist < threshold {
                    // Solo colocar lava si hay soporte debajo (no flotando)
                    if self.is_position_occupied(x, center_y - 1, z) {
                        self.occupied_positions.remove(&(x, center_y, z));
                        self = self.add_cube(x as f32, center_y as f32, z as f32, 1.0, "lava");
                    }
                }
            }
        }
        
        self
    }
    
    pub fn add_nether_reflection(mut self, center_x: i32, center_y: i32, center_z: i32, radius: i32) -> Self {
        let cx = center_x as f32;
        let cy = center_y as f32;
        let cz = center_z as f32;
        let r = radius as f32;
        
        for y in -radius..=radius {
            for x in -radius..=radius {
                for z in -radius..=radius {
                    let fx = x as f32;
                    let fy = y as f32;
                    let fz = z as f32;
                    
                    let dist = ((fx * fx + fy * fy * 1.5 + fz * fz).sqrt()) / r;
                    let density = (1.0 - dist) + (fy / r) * 0.3;
                    let noise = ((fx * 0.3 + fz * 0.5).sin() * (fy * 0.4).cos()) * 0.2;
                    
                    if density + noise > 0.2 {
                        let wx = cx + fx;
                        let wy = cy - fy;
                        let wz = cz + fz;
                        
                        let material = if fy < -radius as f32 * 0.5 {
                            "soul_sand"
                        } else if fy < -radius as f32 * 0.0 {
                            "netherrack"
                        } else {
                            "nether_brick"
                        };
                        
                        self = self.add_cube(wx, wy, wz, 1.0, material);
                    }
                }
            }
        }
        
        self
    }
    
    pub fn add_island_vegetation_auto(
        mut self, 
        center_x: i32, 
        center_z: i32, 
        density: f32
    ) -> Self {
        use std::collections::HashMap;
        
        let mut grass_map: HashMap<(i32, i32), i32> = HashMap::new();
        
        for (x, y, z) in &self.grass_positions {
            grass_map
                .entry((*x, *z))
                .and_modify(|max_y| *max_y = (*max_y).max(*y))
                .or_insert(*y);
        }
        
        let mut tree_count = 0;
        
        for ((x, z), grass_y) in grass_map {
            let hash = ((x * 73856093) ^ (z * 19349663)) as f32;
            let random = (hash.abs() % 1000.0) / 1000.0;
            
            let dx = (x - center_x) as f32;
            let dz = (z - center_z) as f32;
            let dist_to_center = dx * dx + dz * dz;
            let min_dist_sq = 4.0_f32;
            
            // Verificar que no haya agua en esta posición
            let has_water = self.is_position_occupied(x, grass_y + 1, z);
            
            if random < density && dist_to_center > min_dist_sq && !has_water {
                self = self.add_tree(x, grass_y + 1, z);
                tree_count += 1;
            }
        }
        
        println!("🌳 Generados {} árboles", tree_count);
        
        self
    }
    
    // Actualizar add_nether_features para usar lagos orgánicos
    pub fn add_nether_features(mut self, center_x: i32, center_y: i32, center_z: i32, radius: i32) -> Self {
        let bottom_y = center_y - radius;
        
        // Lagos de lava orgánicos
        self = self.add_lava_lake(center_x - 3, bottom_y - 1, center_z - 3, 2);
        self = self.add_lava_lake(center_x + 4, bottom_y - 1, center_z + 4, 3);
        
        // ✅ Pilares de glowstone con tamaño 1.0
        let pillar_positions = [
            (center_x - 4, center_z - 4),
            (center_x + 5, center_z + 4),
        ];
        
        for (x, z) in pillar_positions {
            for h in 0..3 {
                let y = bottom_y - h - 3;
                if !self.is_position_occupied(x, y, z) {
                    self = self.add_cube(x as f32, y as f32, z as f32, 1.0, "glowstone");
                }
            }
        }
        
        self
    }
    
    pub fn add_dual_world_lighting(mut self, center_x: f32, center_z: f32) -> Self {
        self = self.add_sun(center_x + 20.0, 30.0, center_z + 20.0, 4.0);
        self = self.add_light(center_x - 15.0, 25.0, center_z - 15.0, Color::new(180, 200, 255, 255), 2.0);
        self = self.add_light(center_x, -10.0, center_z, Color::new(255, 80, 30, 255), 3.5);
        self = self.add_light(center_x + 5.0, -15.0, center_z + 5.0, Color::new(255, 120, 40, 255), 2.5);
        self = self.add_light(center_x - 5.0, -15.0, center_z - 5.0, Color::new(255, 100, 20, 255), 2.5);
        self
    }
    
    pub fn build(self) -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
        (self.objects, self.lights)
    }
}

pub enum WallDirection {
    North,
    South,
    East,
    West,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self::new()
    }
}