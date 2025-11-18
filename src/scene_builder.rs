use raylib::prelude::*;
use std::sync::Arc;
use crate::ray_intersect::RayIntersect;
use crate::light::Light;
use crate::material::Material;
use crate::cube::Cube;
use crate::mesh::Mesh;

/// Constructor intuitivo de escenas 3D
/// 
/// # Ejemplo
/// ```
/// let scene = SceneBuilder::new()
///     .add_floor(10, "grass_top")
///     .add_cube(0.0, 2.0, 0.0, 2.0, "stone")
///     .add_light(5.0, 10.0, 5.0, Color::WHITE, 3.0)
///     .build();
/// ```
pub struct SceneBuilder {
    objects: Vec<Arc<dyn RayIntersect + Send + Sync>>,
    lights: Vec<Light>,
    materials: MaterialLibrary,
}

/// Biblioteca de materiales predefinidos
struct MaterialLibrary {
    materials: std::collections::HashMap<String, Material>,
}

impl MaterialLibrary {
    fn new() -> Self {
        let mut materials = std::collections::HashMap::new();
        
        // Bloques de Minecraft
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
        
        // Nether
        materials.insert("netherrack".to_string(), Self::netherrack());
        materials.insert("nether_brick".to_string(), Self::nether_brick());
        materials.insert("soul_sand".to_string(), Self::soul_sand());
        
        MaterialLibrary { materials }
    }
    
    fn get(&self, name: &str) -> Material {
        self.materials.get(name)
            .cloned()
            .unwrap_or_else(|| {
                println!("⚠️  Material '{}' no encontrado, usando stone por defecto", name);
                Self::stone()
            })
    }
    
    // === MATERIALES PREDEFINIDOS ===
    
    fn grass_top() -> Material {
        Material::new(
            Vector3::new(0.2, 0.8, 0.2),
            10.0, [0.9, 0.1], 0.0, 0.0,
            Vector3::zero(),
            Some("grass_top".to_string()),
        )
    }
    
    fn grass_side() -> Material {
        Material::new(
            Vector3::new(0.2, 0.8, 0.2),
            10.0, [0.9, 0.1], 0.0, 0.0,
            Vector3::zero(),
            Some("grass_side".to_string()),
        )
    }
    
    fn dirt() -> Material {
        Material::new(
            Vector3::new(0.6, 0.4, 0.2),
            5.0, [0.85, 0.05], 0.0, 0.0,
            Vector3::zero(),
            Some("dirt".to_string()),
        )
    }
    
    fn stone() -> Material {
        Material::new(
            Vector3::new(0.5, 0.5, 0.5),
            15.0, [0.8, 0.15], 0.0, 0.0,
            Vector3::zero(),
            Some("stone".to_string()),
        )
    }
    
    fn wood() -> Material {
        Material::new(
            Vector3::new(0.4, 0.25, 0.1),
            8.0, [0.75, 0.08], 0.0, 0.0,
            Vector3::zero(),
            Some("wood".to_string()),
        )
    }
    
    fn leaves() -> Material {
        Material::new(
            Vector3::new(0.1, 0.5, 0.1),
            5.0, [0.85, 0.05], 0.0, 0.0,
            Vector3::zero(),
            Some("leaves".to_string()),
        )
    }
    
    fn water() -> Material {
        Material::new(
            Vector3::new(0.1, 0.3, 0.7),
            100.0, [0.2, 0.6], 0.0, 0.5,
            Vector3::zero(),
            Some("water".to_string()),
        )
    }
    
    fn lava() -> Material {
        Material::new(
            Vector3::new(1.0, 0.3, 0.0),
            20.0, [0.6, 0.3], 0.0, 0.0,
            Vector3::new(1.0, 0.4, 0.05),
            Some("lava".to_string()),
        )
    }
    
    fn glowstone() -> Material {
        Material::new(
            Vector3::new(1.0, 0.9, 0.6),
            30.0, [0.5, 0.3], 0.0, 0.0,
            Vector3::new(1.5, 1.2, 0.6),
            Some("glowstone".to_string()),
        )
    }
    
    fn glass() -> Material {
        Material::new(
            Vector3::new(0.9, 0.9, 1.0),
            150.0, [0.1, 0.8], 0.7, 0.4,
            Vector3::zero(),
            None,
        )
    }
    
    fn mirror() -> Material {
        Material::new(
            Vector3::new(0.9, 0.9, 1.0),
            120.0, [0.05, 0.6], 0.0, 0.85,
            Vector3::zero(),
            None,
        )
    }
    
    fn gold() -> Material {
        Material::new(
            Vector3::new(1.0, 0.85, 0.0),
            200.0, [0.2, 0.7], 0.0, 0.9,
            Vector3::zero(),
            None,
        )
    }
    
    fn silver() -> Material {
        Material::new(
            Vector3::new(0.95, 0.95, 0.95),
            180.0, [0.15, 0.75], 0.0, 0.95,
            Vector3::zero(),
            None,
        )
    }
    
    fn netherrack() -> Material {
        Material::new(
            Vector3::new(0.6, 0.1, 0.1),
            10.0, [0.8, 0.1], 0.0, 0.0,
            Vector3::zero(),
            Some("netherrack".to_string()),
        )
    }
    
    fn nether_brick() -> Material {
        Material::new(
            Vector3::new(0.2, 0.05, 0.05),
            15.0, [0.7, 0.15], 0.0, 0.0,
            Vector3::zero(),
            Some("nether_brick".to_string()),
        )
    }
    
    fn soul_sand() -> Material {
        Material::new(
            Vector3::new(0.3, 0.2, 0.15),
            8.0, [0.75, 0.1], 0.0, 0.0,
            Vector3::zero(),
            Some("soul_sand".to_string()),
        )
    }
}

impl SceneBuilder {
    /// Crea un nuevo constructor de escenas
    pub fn new() -> Self {
        SceneBuilder {
            objects: Vec::new(),
            lights: Vec::new(),
            materials: MaterialLibrary::new(),
        }
    }
    
    // === MÉTODOS PARA AGREGAR OBJETOS ===
    
    /// Agrega un cubo en la posición especificada
    /// 
    /// # Ejemplo
    /// ```
    /// builder.add_cube(0.0, 2.0, 0.0, 1.0, "stone");
    /// ```
    pub fn add_cube(mut self, x: f32, y: f32, z: f32, size: f32, material: &str) -> Self {
        let mat = self.materials.get(material);
        self.objects.push(Arc::new(Cube::new(
            Vector3::new(x, y, z),
            size,
            mat,
        )));
        self
    }
    
    /// Agrega múltiples cubos en las posiciones especificadas
    pub fn add_cubes(mut self, positions: &[(f32, f32, f32)], size: f32, material: &str) -> Self {
        let mat = self.materials.get(material);
        for (x, y, z) in positions {
            self.objects.push(Arc::new(Cube::new(
                Vector3::new(*x, *y, *z),
                size,
                mat.clone(),
            )));
        }
        self
    }
    
    /// Agrega un piso de cubos (plano XZ)
    /// 
    /// # Ejemplo
    /// ```
    /// builder.add_floor(10, "grass_top"); // Piso de 21x21 cubos
    /// ```
    pub fn add_floor(mut self, radius: i32, material: &str) -> Self {
        let mat = self.materials.get(material);
        for x in -radius..=radius {
            for z in -radius..=radius {
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new(x as f32, 0.0, z as f32),
                    1.0,
                    mat.clone(),
                )));
            }
        }
        self
    }
    
    /// Agrega un piso estilo tablero de ajedrez con dos materiales
    pub fn add_checkered_floor(mut self, radius: i32, material1: &str, material2: &str) -> Self {
        let mat1 = self.materials.get(material1);
        let mat2 = self.materials.get(material2);
        
        for x in -radius..=radius {
            for z in -radius..=radius {
                let mat = if (x + z) % 2 == 0 { mat1.clone() } else { mat2.clone() };
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new(x as f32, 0.0, z as f32),
                    1.0,
                    mat,
                )));
            }
        }
        self
    }
    
    /// Agrega una pared vertical
    pub fn add_wall(mut self, start_x: i32, start_z: i32, length: i32, height: i32, 
                    direction: WallDirection, material: &str) -> Self {
        let mat = self.materials.get(material);
        
        for y in 0..height {
            for i in 0..length {
                let (x, z) = match direction {
                    WallDirection::North => (start_x + i, start_z),
                    WallDirection::South => (start_x + i, start_z),
                    WallDirection::East => (start_x, start_z + i),
                    WallDirection::West => (start_x, start_z + i),
                };
                
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new(x as f32, y as f32, z as f32),
                    1.0,
                    mat.clone(),
                )));
            }
        }
        self
    }
    
    /// Agrega una torre (pilar vertical)
    pub fn add_tower(mut self, x: i32, z: i32, height: i32, material: &str) -> Self {
        let mat = self.materials.get(material);
        for y in 0..height {
            self.objects.push(Arc::new(Cube::new(
                Vector3::new(x as f32, y as f32, z as f32),
                1.0,
                mat.clone(),
            )));
        }
        self
    }
    
    /// Agrega una caja hueca (habitación)
    pub fn add_box(mut self, x: i32, y: i32, z: i32, width: i32, height: i32, depth: i32, material: &str) -> Self {
        let mat = self.materials.get(material);
        
        // Paredes
        for dy in 0..height {
            // Pared frente y atrás
            for dx in 0..width {
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new((x + dx) as f32, (y + dy) as f32, z as f32),
                    1.0, mat.clone(),
                )));
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new((x + dx) as f32, (y + dy) as f32, (z + depth - 1) as f32),
                    1.0, mat.clone(),
                )));
            }
            
            // Pared izquierda y derecha
            for dz in 1..depth-1 {
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new(x as f32, (y + dy) as f32, (z + dz) as f32),
                    1.0, mat.clone(),
                )));
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new((x + width - 1) as f32, (y + dy) as f32, (z + dz) as f32),
                    1.0, mat.clone(),
                )));
            }
        }
        
        // Suelo y techo
        for dx in 0..width {
            for dz in 0..depth {
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new((x + dx) as f32, y as f32, (z + dz) as f32),
                    1.0, mat.clone(),
                )));
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new((x + dx) as f32, (y + height - 1) as f32, (z + dz) as f32),
                    1.0, mat.clone(),
                )));
            }
        }
        
        self
    }
    
    /// Agrega un modelo OBJ cargado desde archivo
    pub fn add_model(mut self, path: &str, x: f32, y: f32, z: f32, scale: f32, material: &str) -> Self {
        let mat = self.materials.get(material);
        
        match Mesh::from_obj(path, &mat, Vector3::new(x, y, z), scale) {
            Ok(mesh) => {
                self.objects.extend(mesh.to_objects());
                println!("Modelo cargado: {}", path);
            }
            Err(e) => {
                println!("Error cargando modelo {}: {}", path, e);
                println!("   Usando cubo como fallback");
                self.objects.push(Arc::new(Cube::new(
                    Vector3::new(x, y, z),
                    scale,
                    mat,
                )));
            }
        }
        self
    }
    
    // === MÉTODOS PARA LUCES ===
    
    /// Agrega una luz puntual
    pub fn add_light(mut self, x: f32, y: f32, z: f32, color: Color, intensity: f32) -> Self {
        self.lights.push(Light::new(
            Vector3::new(x, y, z),
            color,
            intensity,
        ));
        self
    }
    
    /// Agrega una luz blanca (sol)
    pub fn add_sun(self, x: f32, y: f32, z: f32, intensity: f32) -> Self {
        self.add_light(x, y, z, Color::new(255, 250, 240, 255), intensity)
    }
    
    /// Agrega una antorcha (luz naranja + cubo emisivo)
    pub fn add_torch(mut self, x: f32, y: f32, z: f32) -> Self {
        // Cubo emisivo pequeño
        let torch_mat = Material::new(
            Vector3::new(1.0, 0.6, 0.0),
            40.0, [0.3, 0.2], 0.0, 0.0,
            Vector3::new(1.2, 0.6, 0.1),
            Some("glowstone".to_string()),
        );
        
        self.objects.push(Arc::new(Cube::new(
            Vector3::new(x, y, z),
            0.3,
            torch_mat,
        )));
        
        // Luz naranja
        self.lights.push(Light::new(
            Vector3::new(x, y + 0.5, z),
            Color::new(255, 180, 80, 255),
            2.5,
        ));
        
        self
    }
    
    /// Agrega múltiples antorchas
    pub fn add_torches(mut self, positions: &[(f32, f32, f32)]) -> Self {
        for (x, y, z) in positions {
            self = self.add_torch(*x, *y, *z);
        }
        self
    }
    
    // === MÉTODOS PARA ESTRUCTURAS PREDEFINIDAS ===
    
    /// Agrega un árbol simple
    pub fn add_tree(mut self, x: i32, z: i32) -> Self {
        // Tronco
        for y in 0..5 {
            self = self.add_cube(x as f32, y as f32, z as f32, 1.0, "wood");
        }
        
        // Copa de hojas (cruz 3D)
        let leaf_positions = [
            (x, 4, z-1), (x, 4, z+1), (x-1, 4, z), (x+1, 4, z),
            (x, 5, z-1), (x, 5, z+1), (x-1, 5, z), (x+1, 5, z),
            (x, 5, z), (x, 6, z),
        ];
        
        for (lx, ly, lz) in leaf_positions {
            self = self.add_cube(lx as f32, ly as f32, lz as f32, 1.0, "leaves");
        }
        
        self
    }
    
    /// Agrega una casa simple
    pub fn add_house(mut self, x: i32, z: i32) -> Self {
        // Paredes (5x5x3)
        self = self.add_box(x, 0, z, 5, 4, 5, "wood");
        
        // Techo (pirámide simple)
        for level in 0..3 {
            let offset = level;
            for dx in offset..(5-offset) {
                for dz in offset..(5-offset) {
                    self = self.add_cube(
                        (x + dx) as f32,
                        (4 + level) as f32,
                        (z + dz) as f32,
                        1.0,
                        "stone"
                    );
                }
            }
        }
        
        // Puerta (vacío de 2 bloques)
        // (En un builder más avanzado podrías remover bloques)
        
        self
    }
    
    /// Agrega una piscina
    pub fn add_pool(mut self, x: i32, z: i32, width: i32, depth: i32) -> Self {
        // Fondo de piedra
        for dx in 0..width {
            for dz in 0..depth {
                self = self.add_cube((x + dx) as f32, -1.0, (z + dz) as f32, 1.0, "stone");
            }
        }
        
        // Agua
        for dx in 0..width {
            for dz in 0..depth {
                self = self.add_cube((x + dx) as f32, 0.0, (z + dz) as f32, 1.0, "water");
            }
        }
        
        self
    }
    
    // === MÉTODOS DE FINALIZACIÓN ===
    
    /// Construye y retorna la escena final
    pub fn build(self) -> (Vec<Arc<dyn RayIntersect + Send + Sync>>, Vec<Light>) {
        println!("🏗️  Escena construida: {} objetos, {} luces", 
                 self.objects.len(), self.lights.len());
        (self.objects, self.lights)
    }
    
    /// Retorna solo los objetos (sin luces)
    pub fn build_objects(self) -> Vec<Arc<dyn RayIntersect + Send + Sync>> {
        self.objects
    }
    
    /// Retorna solo las luces (sin objetos)
    pub fn build_lights(self) -> Vec<Light> {
        self.lights
    }
}

/// Dirección de una pared
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