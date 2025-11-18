use raylib::prelude::{Color, Vector3};

#[derive(Clone)]
pub struct Material {
    pub diffuse: Vector3,
    pub albedo: [f32; 2],
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub refraction_index: f32,
    pub emissive: Vector3,
    pub texture_path: Option<String>,
}

impl Material {
    pub fn new(
        diffuse: Vector3,
        specular: f32,
        albedo: [f32; 2],
        transparency: f32,
        reflectivity: f32,
        refraction_index: f32,
        emissive: Vector3,
        texture_path: Option<String>,
    ) -> Self {
        Material {
            diffuse,
            albedo,
            specular,
            transparency,
            reflectivity,
            refraction_index,
            emissive,
            texture_path,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Vector3::zero(),
            albedo: [0.0, 0.0],
            specular: 0.0,
            transparency: 0.0,
            reflectivity: 0.0,
            refraction_index: 1.0,
            emissive: Vector3::zero(),
            texture_path: None,
        }
    }
}

pub fn vector3_to_color(v: Vector3) -> Color {
    Color::new(
        (v.x * 255.0).clamp(0.0, 255.0) as u8,
        (v.y * 255.0).clamp(0.0, 255.0) as u8,
        (v.z * 255.0).clamp(0.0, 255.0) as u8,
        255,
    )
}