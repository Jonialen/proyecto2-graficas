use raylib::prelude::Vector3;
use crate::material::Material;

/// Resultado de una intersección rayo-objeto
#[derive(Clone)]
pub struct Intersect {
    pub point: Vector3,
    pub normal: Vector3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

impl Intersect {
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

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect;
    fn get_bounds(&self) -> AABB;
}

/// Axis-Aligned Bounding Box para aceleración espacial
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

impl AABB {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        AABB { min, max }
    }

    pub fn from_points(points: &[Vector3]) -> Self {
        let mut min = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for p in points {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }

        AABB { min, max }
    }

    pub fn union(&self, other: &AABB) -> AABB {
        AABB {
            min: Vector3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vector3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    pub fn center(&self) -> Vector3 {
        (self.min + self.max) * 0.5
    }

    /// Intersección rápida rayo-AABB usando el algoritmo slab method
    pub fn intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> bool {
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

        tmax >= 0.0 && tmin <= tmax
    }
}

/// Bounding Volume Hierarchy para aceleración de ray tracing
pub struct BVH {
    nodes: Vec<BVHNode>,
}

struct BVHNode {
    bounds: AABB,
    left: Option<usize>,
    right: Option<usize>,
    object_index: Option<usize>,
}

impl BVH {
    pub fn build(objects: &[std::sync::Arc<dyn RayIntersect + Send + Sync>]) -> Self {
        if objects.is_empty() {
            return BVH { nodes: Vec::new() };
        }

        let mut nodes = Vec::new();
        let mut primitives: Vec<(AABB, usize)> = objects
            .iter()
            .enumerate()
            .map(|(i, obj)| (obj.get_bounds(), i))
            .collect();

        Self::build_recursive(&mut primitives, &mut nodes, 0);

        BVH { nodes }
    }

    fn build_recursive(
        primitives: &mut [(AABB, usize)],
        nodes: &mut Vec<BVHNode>,
        depth: u32,
    ) -> usize {
        // Calcular bounds de todos los primitivos
        let bounds = primitives
            .iter()
            .fold(primitives[0].0, |acc, (aabb, _)| acc.union(aabb));

        // Caso base: crear hoja
        if primitives.len() == 1 || depth > 32 {
            let node_index = nodes.len();
            nodes.push(BVHNode {
                bounds,
                left: None,
                right: None,
                object_index: Some(primitives[0].1),
            });
            return node_index;
        }

        // Elegir eje de división (el más largo)
        let extent = bounds.max - bounds.min;
        let axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };

        // Ordenar primitivos por el centro en el eje elegido
        primitives.sort_by(|a, b| {
            let center_a = a.0.center();
            let center_b = b.0.center();
            let val_a = match axis {
                0 => center_a.x,
                1 => center_a.y,
                _ => center_a.z,
            };
            let val_b = match axis {
                0 => center_b.x,
                1 => center_b.y,
                _ => center_b.z,
            };
            val_a.partial_cmp(&val_b).unwrap()
        });

        // Dividir en mitades
        let mid = primitives.len() / 2;
        let (left_prims, right_prims) = primitives.split_at_mut(mid);

        // Construir subárboles
        let left = Self::build_recursive(left_prims, nodes, depth + 1);
        let right = Self::build_recursive(right_prims, nodes, depth + 1);

        // Crear nodo interno
        let node_index = nodes.len();
        nodes.push(BVHNode {
            bounds,
            left: Some(left),
            right: Some(right),
            object_index: None,
        });

        node_index
    }

    pub fn intersect(
        &self,
        ray_origin: &Vector3,
        ray_direction: &Vector3,
        objects: &[std::sync::Arc<dyn RayIntersect + Send + Sync>],
    ) -> Intersect {
        if self.nodes.is_empty() {
            return Intersect::empty();
        }

        let mut best_intersect = Intersect::empty();
        let mut best_distance = f32::INFINITY;

        self.intersect_recursive(
            0,
            ray_origin,
            ray_direction,
            objects,
            &mut best_intersect,
            &mut best_distance,
        );

        best_intersect
    }

    fn intersect_recursive(
        &self,
        node_index: usize,
        ray_origin: &Vector3,
        ray_direction: &Vector3,
        objects: &[std::sync::Arc<dyn RayIntersect + Send + Sync>],
        best_intersect: &mut Intersect,
        best_distance: &mut f32,
    ) {
        if node_index >= self.nodes.len() {
            return;
        }

        let node = &self.nodes[node_index];

        // Prueba contra el AABB del nodo
        if !node.bounds.intersect(ray_origin, ray_direction) {
            return;
        }

        // Si es hoja, intersectar con el objeto
        if let Some(obj_idx) = node.object_index {
            if obj_idx < objects.len() {
                let intersect = objects[obj_idx].ray_intersect(ray_origin, ray_direction);
                if intersect.is_intersecting && intersect.distance < *best_distance {
                    *best_distance = intersect.distance;
                    *best_intersect = intersect;
                }
            }
            return;
        }

        // Recursión en hijos
        if let Some(left) = node.left {
            self.intersect_recursive(
                left,
                ray_origin,
                ray_direction,
                objects,
                best_intersect,
                best_distance,
            );
        }

        if let Some(right) = node.right {
            self.intersect_recursive(
                right,
                ray_origin,
                ray_direction,
                objects,
                best_intersect,
                best_distance,
            );
        }
    }
}