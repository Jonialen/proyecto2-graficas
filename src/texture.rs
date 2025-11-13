use raylib::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Sistema de gestión de texturas con placeholders y carga desde archivos
pub struct TextureManager {
    /// Cache de texturas cargadas (nombre -> datos RGBA)
    textures: HashMap<String, TextureData>,
    
    /// Dimensiones por defecto para texturas procedurales
    default_size: u32,
}

/// Datos de una textura cargada
#[derive(Clone)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBA format
}

impl TextureManager {
    /// Crea un nuevo gestor de texturas
    pub fn new() -> Self {
        let mut manager = TextureManager {
            textures: HashMap::new(),
            default_size: 16, // Texturas 16x16 como en Minecraft clásico
        };
        
        // Cargar texturas placeholder por defecto
        manager.load_placeholder_textures();
        
        // Intentar cargar texturas desde assets/textures/
        manager.load_textures_from_directory("assets/textures");
        
        manager
    }

    /// Carga texturas desde un directorio
    pub fn load_textures_from_directory(&mut self, dir_path: &str) {
        let path = Path::new(dir_path);
        
        if !path.exists() {
            println!("⚠️  Directorio de texturas no encontrado: {}", dir_path);
            println!("   Usando texturas procedurales por defecto");
            return;
        }

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                
                if let Some(extension) = file_path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    
                    // Soportar PNG, JPG, BMP, TGA
                    if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "bmp" || ext == "tga" {
                        if let Some(file_name) = file_path.file_stem() {
                            let texture_name = file_name.to_string_lossy().to_string();
                            
                            if self.load_texture_from_file(&texture_name, &file_path.to_string_lossy()) {
                                println!("✅ Textura cargada: {} desde {}", texture_name, file_path.display());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Carga una textura específica desde un archivo
    pub fn load_texture_from_file(&mut self, name: &str, file_path: &str) -> bool {
        // Intentar cargar la imagen usando image crate o manualmente con raylib
        // Por ahora usamos un método simple con lectura de archivos
        
        match self.load_image_data(file_path) {
            Ok(texture_data) => {
                self.textures.insert(name.to_string(), texture_data);
                true
            }
            Err(e) => {
                println!("❌ Error cargando textura {}: {}", file_path, e);
                false
            }
        }
    }

    /// Carga datos de imagen desde un archivo (wrapper simple)
    fn load_image_data(&self, file_path: &str) -> Result<TextureData, String> {
        // Usar image crate para cargar la imagen
        use image::GenericImageView;
        
        let img = image::open(file_path)
            .map_err(|e| format!("No se pudo abrir imagen: {}", e))?;
        
        let (width, height) = img.dimensions();
        let rgba = img.to_rgba8();
        
        Ok(TextureData {
            width,
            height,
            data: rgba.into_raw(),
        })
    }

    /// Genera texturas placeholder con patrones únicos
    fn load_placeholder_textures(&mut self) {
        let size = self.default_size;
        
        // Placeholder para materiales comunes de Minecraft
        self.register_procedural("grass_top", size, size, self.generate_grass_top());
        self.register_procedural("grass_side", size, size, self.generate_grass_side());
        self.register_procedural("dirt", size, size, self.generate_dirt());
        self.register_procedural("stone", size, size, self.generate_stone());
        self.register_procedural("wood", size, size, self.generate_wood());
        self.register_procedural("leaves", size, size, self.generate_leaves());
        self.register_procedural("water", size, size, self.generate_water());
        self.register_procedural("lava", size, size, self.generate_lava());
        self.register_procedural("netherrack", size, size, self.generate_netherrack());
        self.register_procedural("nether_brick", size, size, self.generate_nether_brick());
        self.register_procedural("soul_sand", size, size, self.generate_soul_sand());
        self.register_procedural("glowstone", size, size, self.generate_glowstone());
        
        println!("✨ {} texturas procedurales cargadas", self.textures.len());
    }

    /// Obtiene el color de una textura en coordenadas UV
    pub fn sample(&self, texture_name: &str, u: f32, v: f32) -> Vector3 {
        if let Some(texture_data) = self.textures.get(texture_name) {
            let width = texture_data.width as f32;
            let height = texture_data.height as f32;
            
            // Convertir UV (0-1) a coordenadas de texel con wrapping
            let x = ((u * width) as u32 % texture_data.width) as usize;
            let y = ((v * height) as u32 % texture_data.height) as usize;
            
            let idx = (y * texture_data.width as usize + x) * 4;
            
            if idx + 2 < texture_data.data.len() {
                return Vector3::new(
                    texture_data.data[idx] as f32 / 255.0,
                    texture_data.data[idx + 1] as f32 / 255.0,
                    texture_data.data[idx + 2] as f32 / 255.0,
                );
            }
        }
        
        // Fallback: patrón de tablero de ajedrez rosa/negro (indica textura faltante)
        let checker = ((u * 8.0) as i32 + (v * 8.0) as i32) % 2;
        if checker == 0 {
            Vector3::new(1.0, 0.0, 1.0) // Magenta
        } else {
            Vector3::new(0.0, 0.0, 0.0) // Negro
        }
    }

    /// Registra una textura procedural manualmente
    pub fn register_procedural(&mut self, name: &str, width: u32, height: u32, data: Vec<u8>) {
        self.textures.insert(
            name.to_string(),
            TextureData {
                width,
                height,
                data,
            },
        );
    }

    // ===== GENERADORES DE TEXTURAS PLACEHOLDER =====

    fn generate_grass_top(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                // Verde con variación aleatoria
                let noise = ((x * 7 + y * 13) % 5) as u8;
                data.push(50 + noise);   // R
                data.push(180 + noise);  // G
                data.push(50 + noise);   // B
                data.push(255);          // A
            }
        }
        data
    }

    fn generate_grass_side(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                if y < self.default_size / 4 {
                    // Parte superior verde
                    let noise = ((x * 7 + y * 13) % 5) as u8;
                    data.push(50 + noise);
                    data.push(180 + noise);
                    data.push(50 + noise);
                    data.push(255);
                } else {
                    // Parte inferior café (tierra)
                    let noise = ((x * 11 + y * 17) % 8) as u8;
                    data.push(130 + noise);
                    data.push(80 + noise);
                    data.push(40 + noise);
                    data.push(255);
                }
            }
        }
        data
    }

    fn generate_dirt(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let noise = ((x * 11 + y * 17) % 12) as u8;
                data.push(130 + noise);
                data.push(80 + noise);
                data.push(40 + noise);
                data.push(255);
            }
        }
        data
    }

    fn generate_stone(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let noise = ((x * 13 + y * 19) % 15) as u8;
                data.push(100 + noise);
                data.push(100 + noise);
                data.push(100 + noise);
                data.push(255);
            }
        }
        data
    }

    fn generate_wood(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                // Patrón de anillos
                let ring = (x as i32 - 8).abs() + (y as i32 - 8).abs();
                let noise = (ring % 4) as u8 * 10;
                data.push(80 + noise);
                data.push(50 + noise / 2);
                data.push(20 + noise / 3);
                data.push(255);
            }
        }
        data
    }

    fn generate_leaves(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let noise = ((x * 23 + y * 29) % 20) as u8;
                data.push(20 + noise / 2);
                data.push(120 + noise);
                data.push(20 + noise / 2);
                data.push(255);
            }
        }
        data
    }

    fn generate_water(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let wave = ((x + y) % 4) as u8 * 5;
                data.push(20 + wave);
                data.push(60 + wave);
                data.push(180 + wave);
                data.push(200); // Semi-transparente
            }
        }
        data
    }

    fn generate_lava(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let flow = ((x * 3 + y * 7) % 10) as u8 * 8;
                data.push(255);
                data.push(100 + flow);
                data.push(flow / 2);
                data.push(255);
            }
        }
        data
    }

    fn generate_netherrack(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let noise = ((x * 31 + y * 37) % 25) as u8;
                data.push(150 + noise);
                data.push(40 + noise / 2);
                data.push(40 + noise / 2);
                data.push(255);
            }
        }
        data
    }

    fn generate_nether_brick(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                // Patrón de ladrillos
                let is_mortar = (x % 8 == 0) || (y % 8 == 0);
                if is_mortar {
                    data.push(20);
                    data.push(10);
                    data.push(10);
                } else {
                    data.push(50);
                    data.push(15);
                    data.push(15);
                }
                data.push(255);
            }
        }
        data
    }

    fn generate_soul_sand(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let noise = ((x * 41 + y * 43) % 18) as u8;
                data.push(70 + noise);
                data.push(50 + noise);
                data.push(35 + noise);
                data.push(255);
            }
        }
        data
    }

    fn generate_glowstone(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let glow = ((x + y) % 3) as u8 * 15;
                data.push(255);
                data.push(220 + glow);
                data.push(100 + glow);
                data.push(255);
            }
        }
        data
    }
}