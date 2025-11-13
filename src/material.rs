use raylib::prelude::{Color, Vector3};

/// Representa las propiedades físicas y visuales de un material
#[derive(Clone)]
pub struct Material {
    /// Color base del material (albedo RGB)
    pub diffuse: Vector3,
    
    /// Componentes de reflectancia: [difuso, especular]
    pub albedo: [f32; 2],
    
    /// Exponente especular (brillo) - valores altos = superficies más brillantes
    pub specular: f32,
    
    /// Coeficiente de transparencia (0.0 = opaco, 1.0 = transparente)
    pub transparency: f32,
    
    /// Coeficiente de reflexión (0.0 = no refleja, 1.0 = espejo perfecto)
    pub reflectivity: f32,
    
    /// Color de emisión para materiales que emiten luz (antorchas, glowstone, etc.)
    pub emissive: Vector3,
    
    /// Ruta de textura para futuras extensiones
    /// TODO: Implementar carga de textura con raylib::Texture2D
    /// En versiones futuras, esto permitirá cargar texturas de bloques estilo Minecraft
    pub texture_path: Option<String>,
}

impl Material {
    /// Crea un nuevo material con todas las propiedades especificadas
    pub fn new(
        diffuse: Vector3,
        specular: f32,
        albedo: [f32; 2],
        transparency: f32,
        reflectivity: f32,
        emissive: Vector3,
        texture_path: Option<String>,
    ) -> Self {
        Material {
            diffuse,
            albedo,
            specular,
            transparency,
            reflectivity,
            emissive,
            texture_path,
        }
    }

    /// Crea un material negro por defecto (usado para intersecciones vacías)
    pub fn black() -> Self {
        Material {
            diffuse: Vector3::zero(),
            albedo: [0.0, 0.0],
            specular: 0.0,
            transparency: 0.0,
            reflectivity: 0.0,
            emissive: Vector3::zero(),
            texture_path: None,
        }
    }
}

/// Convierte un Vector3 (RGB normalizado) a Color de raylib
pub fn vector3_to_color(v: Vector3) -> Color {
    Color::new(
        (v.x * 255.0).clamp(0.0, 255.0) as u8,
        (v.y * 255.0).clamp(0.0, 255.0) as u8,
        (v.z * 255.0).clamp(0.0, 255.0) as u8,
        255,
    )
}