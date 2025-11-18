use raylib::prelude::*;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub struct AnimatedTexture {
    frames: Vec<TextureData>,
    frame_duration: f32,
}

pub struct TextureManager {
    textures: HashMap<String, TextureData>,
    animated_textures: HashMap<String, AnimatedTexture>,
    default_size: u32,
    time: f32,
}

impl TextureManager {
    pub fn new() -> Self {
        let mut manager = TextureManager {
            textures: HashMap::new(),
            animated_textures: HashMap::new(),
            default_size: 16,
            time: 0.0,
        };
        
        // 1. Generar texturas procedurales en memoria
        manager.load_placeholder_textures();
        manager.load_animated_textures();
        
        // 2. Intentar cargar desde disco (reemplaza las procedurales si existen)
        manager.load_textures_from_directory("assets/textures");
        
        // ✅ 3. Exportar las que faltan (NUEVO)
        manager.export_missing_textures("assets/textures");
        
        manager
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
    }
    
    pub fn export_missing_textures(&self, dir_path: &str) {
        // Crear el directorio si no existe
        if let Err(e) = std::fs::create_dir_all(dir_path) {
            println!("⚠️  No se pudo crear directorio {}: {}", dir_path, e);
            return;
        }

        let mut exported_count = 0;
        let mut skipped_count = 0;

        // Exportar texturas estáticas
        for (name, texture_data) in &self.textures {
            let file_path = format!("{}/{}.png", dir_path, name);
            
            if Path::new(&file_path).exists() {
                skipped_count += 1;
                continue;
            }

            if self.export_texture_to_file(name, &file_path, texture_data) {
                println!("💾 Textura exportada: {}", file_path);
                exported_count += 1;
            }
        }

        // Exportar frames de texturas animadas
        for (name, animated) in &self.animated_textures {
            for (frame_idx, frame_data) in animated.frames.iter().enumerate() {
                let file_path = format!("{}/{}_{}.png", dir_path, name, frame_idx);
                
                if Path::new(&file_path).exists() {
                    skipped_count += 1;
                    continue;
                }

                if self.export_texture_to_file(
                    &format!("{}_{}", name, frame_idx),
                    &file_path,
                    frame_data
                ) {
                    println!("💾 Frame de animación exportado: {}", file_path);
                    exported_count += 1;
                }
            }
        }

        if exported_count > 0 {
            println!("✅ {} texturas exportadas a {}", exported_count, dir_path);
        }
        if skipped_count > 0 {
            println!("⏭️  {} texturas ya existían (no sobreescritas)", skipped_count);
        }
    }

    /// Exporta una textura individual a un archivo PNG
    fn export_texture_to_file(
        &self,
        _name: &str,
        file_path: &str,
        texture_data: &TextureData
    ) -> bool {
        match image::RgbaImage::from_raw(
            texture_data.width,
            texture_data.height,
            texture_data.data.clone()
        ) {
            Some(img) => {
                match img.save(file_path) {
                    Ok(_) => true,
                    Err(e) => {
                        println!("❌ Error guardando {}: {}", file_path, e);
                        false
                    }
                }
            }
            _none => {
                println!("❌ Error creando imagen para {}", file_path);
                false
            }
        }
    }

    #[allow(dead_code)]
    pub fn export_texture(&self, name: &str, output_path: &str) -> bool {
        if let Some(texture_data) = self.textures.get(name) {
            return self.export_texture_to_file(name, output_path, texture_data);
        }

        println!("⚠️  Textura '{}' no encontrada", name);
        false
    }

    #[allow(dead_code)]
    pub fn export_all_textures(&self, dir_path: &str) -> usize {
        if let Err(e) = std::fs::create_dir_all(dir_path) {
            println!("❌ No se pudo crear directorio {}: {}", dir_path, e);
            return 0;
        }

        let mut count = 0;

        // Exportar texturas estáticas
        for (name, texture_data) in &self.textures {
            let file_path = format!("{}/{}.png", dir_path, name);
            if self.export_texture_to_file(name, &file_path, texture_data) {
                count += 1;
            }
        }

        // Exportar frames animados
        for (name, animated) in &self.animated_textures {
            for (frame_idx, frame_data) in animated.frames.iter().enumerate() {
                let file_path = format!("{}/{}_{}.png", dir_path, name, frame_idx);
                if self.export_texture_to_file(
                    &format!("{}_{}", name, frame_idx),
                    &file_path,
                    frame_data
                ) {
                    count += 1;
                }
            }
        }

        println!("✅ {} texturas exportadas (todas) a {}", count, dir_path);
        count
    }

    pub fn load_textures_from_directory(&mut self, dir_path: &str) {
        let path = Path::new(dir_path);
        
        if !path.exists() {
            println!("📂 Directorio de texturas no existe aún: {}", dir_path);
            println!("   Se crearán texturas procedurales");
            return;
        }

        let mut loaded_count = 0;

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                
                if let Some(extension) = file_path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    
                    if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "bmp" || ext == "tga" {
                        if let Some(file_name) = file_path.file_stem() {
                            let texture_name = file_name.to_string_lossy().to_string();
                            
                            // Verificar si es un frame de animación
                            if let Some(base_name) = texture_name.strip_suffix("_0")
                                .or_else(|| texture_name.strip_suffix("_1"))
                                .or_else(|| texture_name.strip_suffix("_2"))
                                .or_else(|| texture_name.strip_suffix("_3"))
                                .or_else(|| texture_name.strip_suffix("_4"))
                                .or_else(|| texture_name.strip_suffix("_5"))
                            {
                                // Es un frame de animación, cargar todos los frames
                                self.load_animated_texture_from_files(base_name, dir_path);
                            } else if self.load_texture_from_file(&texture_name, &file_path.to_string_lossy()) {
                                loaded_count += 1;
                            }
                        }
                    }
                }
            }
        }

        if loaded_count > 0 {
            println!("✅ {} texturas cargadas desde {}", loaded_count, dir_path);
        }
    }

    /// Carga todos los frames de una textura animada desde archivos
    fn load_animated_texture_from_files(&mut self, base_name: &str, dir_path: &str) -> bool {
        let mut frames = Vec::new();
        let mut frame_idx = 0;

        loop {
            let file_path = format!("{}/{}_{}.png", dir_path, base_name, frame_idx);
            
            if !Path::new(&file_path).exists() {
                break;
            }

            match self.load_image_data(&file_path) {
                Ok(texture_data) => {
                    frames.push(texture_data);
                    frame_idx += 1;
                }
                Err(_) => break,
            }
        }

        if !frames.is_empty() {
            let frame_duration = match base_name {
                "water" => 0.3,
                "lava" => 0.2,
                "portal" => 0.15,
                _ => 0.25,
            };

            self.animated_textures.insert(
                base_name.to_string(),
                AnimatedTexture {
                    frames,
                    frame_duration,
                },
            );

            println!("🎬 Textura animada cargada: {} ({} frames)", base_name, frame_idx);
            true
        } else {
            false
        }
    }

    pub fn load_texture_from_file(&mut self, name: &str, file_path: &str) -> bool {
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

    fn load_image_data(&self, file_path: &str) -> Result<TextureData, String> {
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

    fn load_animated_textures(&mut self) {
        let water_frames = vec![
            self.generate_water_frame(0),
            self.generate_water_frame(1),
            self.generate_water_frame(2),
            self.generate_water_frame(3),
        ];
        
        self.animated_textures.insert(
            "water".to_string(),
            AnimatedTexture {
                frames: water_frames.into_iter().map(|data| TextureData {
                    width: self.default_size,
                    height: self.default_size,
                    data,
                }).collect(),
                frame_duration: 0.3,
            },
        );

        let lava_frames = vec![
            self.generate_lava_frame(0),
            self.generate_lava_frame(1),
            self.generate_lava_frame(2),
            self.generate_lava_frame(3),
        ];
        
        self.animated_textures.insert(
            "lava".to_string(),
            AnimatedTexture {
                frames: lava_frames.into_iter().map(|data| TextureData {
                    width: self.default_size,
                    height: self.default_size,
                    data,
                }).collect(),
                frame_duration: 0.2,
            },
        );

        let portal_frames = vec![
            self.generate_portal_frame(0),
            self.generate_portal_frame(1),
            self.generate_portal_frame(2),
            self.generate_portal_frame(3),
            self.generate_portal_frame(4),
            self.generate_portal_frame(5),
        ];
        
        self.animated_textures.insert(
            "portal".to_string(),
            AnimatedTexture {
                frames: portal_frames.into_iter().map(|data| TextureData {
                    width: self.default_size,
                    height: self.default_size,
                    data,
                }).collect(),
                frame_duration: 0.15,
            },
        );
    }

    fn load_placeholder_textures(&mut self) {
        let size = self.default_size;
        
        self.register_procedural("grass_top", size, size, self.generate_grass_top());
        self.register_procedural("grass_side", size, size, self.generate_grass_side());
        self.register_procedural("dirt", size, size, self.generate_dirt());
        self.register_procedural("stone", size, size, self.generate_stone());
        self.register_procedural("wood", size, size, self.generate_wood());
        self.register_procedural("leaves", size, size, self.generate_leaves());
        self.register_procedural("netherrack", size, size, self.generate_netherrack());
        self.register_procedural("nether_brick", size, size, self.generate_nether_brick());
        self.register_procedural("soul_sand", size, size, self.generate_soul_sand());
        self.register_procedural("glowstone", size, size, self.generate_glowstone());
        self.register_procedural("diamond", size, size, self.generate_diamond());
        self.register_procedural("emerald", size, size, self.generate_emerald());
        self.register_procedural("obsidian", size, size, self.generate_obsidian());
        self.register_procedural("ice", size, size, self.generate_ice());
        
        println!("✨ {} texturas procedurales cargadas", self.textures.len());
    }

    pub fn sample(&self, texture_name: &str, u: f32, v: f32) -> Vector3 {
        if let Some(animated) = self.animated_textures.get(texture_name) {
            let total_frames = animated.frames.len() as f32;
            let frame_index = ((self.time / animated.frame_duration) % total_frames) as usize;
            let texture_data = &animated.frames[frame_index];
            
            let width = texture_data.width as f32;
            let height = texture_data.height as f32;
            
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

        if let Some(texture_data) = self.textures.get(texture_name) {
            let width = texture_data.width as f32;
            let height = texture_data.height as f32;
            
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
        
        let checker = ((u * 8.0) as i32 + (v * 8.0) as i32) % 2;
        if checker == 0 {
            Vector3::new(1.0, 0.0, 1.0)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }

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

    fn generate_grass_top(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let noise = ((x * 7 + y * 13) % 5) as u8;
                data.push(50 + noise);
                data.push(180 + noise);
                data.push(50 + noise);
                data.push(255);
            }
        }
        data
    }

    fn generate_grass_side(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                if y < self.default_size / 4 {
                    let noise = ((x * 7 + y * 13) % 5) as u8;
                    data.push(50 + noise);
                    data.push(180 + noise);
                    data.push(50 + noise);
                    data.push(255);
                } else {
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

    fn generate_water_frame(&self, frame: i32) -> Vec<u8> {
        let mut data = Vec::new();
        let offset = frame as u32 * 2;
        
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let wave = (((x + offset) + y) % 4) as u8 * 8;
                data.push(20 + wave);
                data.push(60 + wave);
                data.push(180 + wave);
                data.push(200);
            }
        }
        data
    }

    fn generate_lava_frame(&self, frame: i32) -> Vec<u8> {
        let mut data = Vec::new();
        let offset = (frame * 17) as u32;
        
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let flow = ((x * 3 + y * 7 + offset) % 12) as u8 * 10;
                let bubble = if (x + y + offset / 2) % 7 == 0 { 40 } else { 0 };
                data.push(255);
                data.push(100 + flow + bubble);
                data.push((flow / 2) + bubble / 2);
                data.push(255);
            }
        }
        data
    }

    fn generate_portal_frame(&self, frame: i32) -> Vec<u8> {
        let mut data = Vec::new();
        let phase = frame as f32 * 0.5;
        
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let wave = (((x as f32 * 0.8 + phase).sin() + (y as f32 * 0.6 + phase).cos()) * 50.0) as i32;
                let swirl = (((x as f32 - 8.0).atan2(y as f32 - 8.0) * 3.0 + phase).sin() * 30.0) as i32;
                let purple = (128 + wave + swirl).clamp(80, 220) as u8;
                let magenta = (0 + wave / 3 + swirl / 2).clamp(0, 120) as u8;
                let violet = (200 + wave + swirl).clamp(150, 255) as u8;
                
                data.push(purple);
                data.push(magenta);
                data.push(violet);
                data.push(180);
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

    fn generate_diamond(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let crystal = ((x + y) % 3) as u8;
                let sparkle = if (x * 7 + y * 11) % 13 == 0 { 60 } else { 0 };
                data.push(180 + crystal * 20 + sparkle);
                data.push(230 + crystal * 8 + sparkle);
                data.push(255);
                data.push(255);
            }
        }
        data
    }

    fn generate_emerald(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let crystal = ((x + y) % 4) as u8;
                let sparkle = if (x * 5 + y * 13) % 11 == 0 { 40 } else { 0 };
                data.push(50 + crystal * 10 + sparkle);
                data.push(230 + crystal * 6 + sparkle);
                data.push(80 + crystal * 15 + sparkle);
                data.push(255);
            }
        }
        data
    }

    fn generate_obsidian(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let noise = ((x * 17 + y * 23) % 20) as u8;
                let purple_tint = if noise > 15 { 20 } else { 0 };
                data.push(10 + noise / 2);
                data.push(5 + noise / 4);
                data.push(25 + purple_tint);
                data.push(255);
            }
        }
        data
    }

    fn generate_ice(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for y in 0..self.default_size {
            for x in 0..self.default_size {
                let crack = if (x + y) % 7 == 0 { 30 } else { 0 };
                let shine = ((x * y) % 5) as u8 * 3;
                data.push(200 + shine + crack);
                data.push(230 + shine + crack);
                data.push(255);
                data.push(200);
            }
        }
        data
    }
}