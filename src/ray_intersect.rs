use raylib::prelude::Vector3;
use crate::material::Material;

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

pub struct BVH {
    root: Option<Box<BVHNode>>,
}

enum BVHNode {
    Leaf {
        bounds: AABB,
        object_index: usize,
    },
    Internal {
        bounds: AABB,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    },
}

impl BVH {
    pub fn build(objects: &[std::sync::Arc<dyn RayIntersect + Send + Sync>]) -> Self {
        if objects.is_empty() {
            return BVH { root: None };
        }

        let mut primitives: Vec<(AABB, usize)> = objects
            .iter()
            .enumerate()
            .map(|(i, obj)| (obj.get_bounds(), i))
            .collect();

        let root = Self::build_recursive(&mut primitives, 0);
        
        BVH { root: Some(Box::new(root)) }
    }

    fn build_recursive(primitives: &mut [(AABB, usize)], depth: u32) -> BVHNode {
        let bounds = primitives
            .iter()
            .fold(primitives[0].0, |acc, (aabb, _)| acc.union(aabb));

        if primitives.len() == 1 {
            return BVHNode::Leaf {
                bounds,
                object_index: primitives[0].1,
            };
        }

        if depth > 50 {
            return BVHNode::Leaf {
                bounds,
                object_index: primitives[0].1,
            };
        }

        let extent = bounds.max - bounds.min;
        let axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };

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
            val_a.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mid = primitives.len() / 2;
        let mid = mid.max(1).min(primitives.len() - 1);
        
        let (left_slice, right_slice) = primitives.split_at_mut(mid);
        
        let left = Self::build_recursive(left_slice, depth + 1);
        let right = Self::build_recursive(right_slice, depth + 1);

        BVHNode::Internal {
            bounds,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn intersect(
        &self,
        ray_origin: &Vector3,
        ray_direction: &Vector3,
        objects: &[std::sync::Arc<dyn RayIntersect + Send + Sync>],
    ) -> Intersect {
        if let Some(root) = &self.root {
            let mut best_intersect = Intersect::empty();
            let mut best_distance = f32::INFINITY;
            
            Self::intersect_node(
                root,
                ray_origin,
                ray_direction,
                objects,
                &mut best_intersect,
                &mut best_distance,
            );
            
            best_intersect
        } else {
            Intersect::empty()
        }
    }

    fn intersect_node(
        node: &BVHNode,
        ray_origin: &Vector3,
        ray_direction: &Vector3,
        objects: &[std::sync::Arc<dyn RayIntersect + Send + Sync>],
        best_intersect: &mut Intersect,
        best_distance: &mut f32,
    ) {
        match node {
            BVHNode::Leaf { bounds, object_index } => {
                if !bounds.intersect(ray_origin, ray_direction) {
                    return;
                }
                
                if *object_index < objects.len() {
                    let intersect = objects[*object_index].ray_intersect(ray_origin, ray_direction);
                    if intersect.is_intersecting && intersect.distance < *best_distance {
                        *best_distance = intersect.distance;
                        *best_intersect = intersect;
                    }
                }
            }
            BVHNode::Internal { bounds, left, right } => {
                if !bounds.intersect(ray_origin, ray_direction) {
                    return;
                }
                
                Self::intersect_node(left, ray_origin, ray_direction, objects, best_intersect, best_distance);
                Self::intersect_node(right, ray_origin, ray_direction, objects, best_intersect, best_distance);
            }
        }
    }
}