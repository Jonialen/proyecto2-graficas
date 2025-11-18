use raylib::prelude::Vector3;
use crate::ray_intersect::{Intersect, RayIntersect, AABB};
use crate::material::Material;

/// Cubo alineado a los ejes con cache optimizado
pub struct Cube {
    pub center: Vector3,
    pub size: f32,
    pub material: Material,
    min: Vector3,
    max: Vector3,
    half_size: f32,
    inv_size: f32,
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
            half_size,
            inv_size: 1.0 / size,
        }
    }

    #[inline]
    fn get_uv(&self, point: &Vector3, normal: &Vector3) -> (f32, f32) {
        let local = (*point - self.center) * self.inv_size;

        let (u, v) = if normal.x.abs() > 0.9 {
            ((local.z + 1.0) * 0.5, (local.y + 1.0) * 0.5)
        } else if normal.y.abs() > 0.9 {
            ((local.x + 1.0) * 0.5, (local.z + 1.0) * 0.5)
        } else {
            ((local.x + 1.0) * 0.5, (local.y + 1.0) * 0.5)
        };

        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
}

impl RayIntersect for Cube {
    #[inline]
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        // Slab method optimizado con early exit
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
        let local = point - self.center;
        
        // Determinar normal basándose en la cara más cercana
        let abs_x = local.x.abs();
        let abs_y = local.y.abs();
        let abs_z = local.z.abs();
        
        let normal = if abs_x > abs_y && abs_x > abs_z {
            Vector3::new(local.x.signum(), 0.0, 0.0)
        } else if abs_y > abs_z {
            Vector3::new(0.0, local.y.signum(), 0.0)
        } else {
            Vector3::new(0.0, 0.0, local.z.signum())
        };

        let (u, v) = self.get_uv(&point, &normal);

        Intersect::new(point, normal, t, self.material.clone(), u, v)
    }

    fn get_bounds(&self) -> AABB {
        AABB::new(self.min, self.max)
    }
}