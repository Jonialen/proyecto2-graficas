use raylib::prelude::Vector3;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::material::Material;

/// Representa un cubo alineado a los ejes (AABB - Axis-Aligned Bounding Box)
pub struct Cube {
    pub center: Vector3,
    pub size: f32,
    pub material: Material,
    // Cache para optimización
    min: Vector3,
    max: Vector3,
}

impl Cube {
    pub fn new(center: Vector3, size: f32, material: Material) -> Self {
        let half_size = size / 2.0;
        let min = center - Vector3::new(half_size, half_size, half_size);
        let max = center + Vector3::new(half_size, half_size, half_size);
        
        Cube {
            center,
            size,
            material,
            min,
            max,
        }
    }

    /// Calcula las coordenadas UV basándose en qué cara del cubo fue intersectada
    fn get_uv(&self, point: &Vector3, normal: &Vector3) -> (f32, f32) {
        let half_size = self.size / 2.0;
        let local = *point - self.center;

        // Determinar qué cara fue golpeada basándose en la normal
        let (u, v) = if normal.x.abs() > 0.9 {
            // Cara X (izquierda o derecha)
            ((local.z / half_size + 1.0) / 2.0, (local.y / half_size + 1.0) / 2.0)
        } else if normal.y.abs() > 0.9 {
            // Cara Y (arriba o abajo)
            ((local.x / half_size + 1.0) / 2.0, (local.z / half_size + 1.0) / 2.0)
        } else {
            // Cara Z (frente o atrás)
            ((local.x / half_size + 1.0) / 2.0, (local.y / half_size + 1.0) / 2.0)
        };

        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        // Algoritmo optimizado de intersección rayo-AABB (slab method)
        let inv_dir = Vector3::new(
            1.0 / ray_direction.x,
            1.0 / ray_direction.y,
            1.0 / ray_direction.z,
        );

        let t1 = (self.min.x - ray_origin.x) * inv_dir.x;
        let t2 = (self.max.x - ray_origin.x) * inv_dir.x;
        let t3 = (self.min.y - ray_origin.y) * inv_dir.y;
        let t4 = (self.max.y - ray_origin.y) * inv_dir.y;
        let t5 = (self.min.z - ray_origin.z) * inv_dir.z;
        let t6 = (self.max.z - ray_origin.z) * inv_dir.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 || tmin > tmax {
            return Intersect::empty();
        }

        let t = if tmin > 0.0 { tmin } else { tmax };

        if t < 0.0 {
            return Intersect::empty();
        }

        let point = *ray_origin + *ray_direction * t;
        
        // Calcular la normal basándose en qué cara fue golpeada
        let local = point - self.center;
        let half_size = self.size / 2.0;
        let epsilon = 0.001;
        
        let normal = if (local.x - half_size).abs() < epsilon {
            Vector3::new(1.0, 0.0, 0.0)
        } else if (local.x + half_size).abs() < epsilon {
            Vector3::new(-1.0, 0.0, 0.0)
        } else if (local.y - half_size).abs() < epsilon {
            Vector3::new(0.0, 1.0, 0.0)
        } else if (local.y + half_size).abs() < epsilon {
            Vector3::new(0.0, -1.0, 0.0)
        } else if (local.z - half_size).abs() < epsilon {
            Vector3::new(0.0, 0.0, 1.0)
        } else {
            Vector3::new(0.0, 0.0, -1.0)
        };

        let (u, v) = self.get_uv(&point, &normal);

        Intersect::new(point, normal, t, self.material.clone(), u, v)
    }
}