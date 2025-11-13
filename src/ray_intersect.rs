use raylib::prelude::Vector3;
use crate::material::Material;

/// Resultado de una intersección rayo-objeto
#[derive(Clone)]
pub struct Intersect {
    /// Punto de intersección en coordenadas del mundo
    pub point: Vector3,
    
    /// Normal de la superficie en el punto de intersección
    pub normal: Vector3,
    
    /// Distancia desde el origen del rayo hasta el punto de intersección
    pub distance: f32,
    
    /// Indica si hubo una intersección válida
    pub is_intersecting: bool,
    
    /// Material del objeto intersectado
    pub material: Material,
    
    /// Coordenadas de textura U (horizontal)
    pub u: f32,
    
    /// Coordenadas de textura V (vertical)
    pub v: f32,
}

impl Intersect {
    /// Crea una nueva intersección válida
    pub fn new(
        point: Vector3,
        normal: Vector3,
        distance: f32,
        material: Material,
        u: f32,
        v: f32,
    ) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
            u,
            v,
        }
    }

    /// Crea una intersección vacía (sin colisión)
    pub fn empty() -> Self {
        Intersect {
            point: Vector3::zero(),
            normal: Vector3::zero(),
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
            u: 0.0,
            v: 0.0,
        }
    }
}

/// Trait que debe implementar cualquier objeto que pueda ser intersectado por un rayo
/// Debe ser thread-safe para paralelización con rayon
pub trait RayIntersect {
    /// Calcula la intersección entre un rayo y el objeto
    /// 
    /// # Argumentos
    /// * `ray_origin` - Origen del rayo en coordenadas del mundo
    /// * `ray_direction` - Dirección del rayo (debe estar normalizada)
    /// 
    /// # Retorna
    /// Una estructura `Intersect` con información de la intersección
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect;
}