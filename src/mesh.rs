use raylib::prelude::Vector3;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::material::Material;
use std::sync::Arc;

/// Representa un triángulo individual de una malla
pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub n0: Vector3,
    pub n1: Vector3,
    pub n2: Vector3,
    pub uv0: (f32, f32),
    pub uv1: (f32, f32),
    pub uv2: (f32, f32),
    pub material: Material,
}

impl Triangle {
    /// Calcula las coordenadas baricéntricas para interpolación
    fn barycentric(&self, point: &Vector3) -> (f32, f32, f32) {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let v0p = *point - self.v0;

        let d00 = v0v1.dot(v0v1);
        let d01 = v0v1.dot(v0v2);
        let d11 = v0v2.dot(v0v2);
        let d20 = v0p.dot(v0v1);
        let d21 = v0p.dot(v0v2);

        let denom = d00 * d11 - d01 * d01;
        
        if denom.abs() < 1e-8 {
            return (1.0, 0.0, 0.0);
        }

        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        (u, v, w)
    }
}

impl RayIntersect for Triangle {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        // Algoritmo de Möller-Trumbore para intersección rayo-triángulo
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        
        let h = ray_direction.cross(edge2);
        let a = edge1.dot(h);

        // Si a está cerca de 0, el rayo es paralelo al triángulo
        if a.abs() < 1e-8 {
            return Intersect::empty();
        }

        let f = 1.0 / a;
        let s = *ray_origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return Intersect::empty();
        }

        let q = s.cross(edge1);
        let v = f * ray_direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return Intersect::empty();
        }

        let t = f * edge2.dot(q);

        if t < 1e-8 {
            return Intersect::empty();
        }

        let point = *ray_origin + *ray_direction * t;
        let (u_bary, v_bary, w_bary) = self.barycentric(&point);

        // Interpolar normal
        let normal = (self.n0 * u_bary + self.n1 * v_bary + self.n2 * w_bary).normalized();

        // Interpolar coordenadas UV
        let uv_u = self.uv0.0 * u_bary + self.uv1.0 * v_bary + self.uv2.0 * w_bary;
        let uv_v = self.uv0.1 * u_bary + self.uv1.1 * v_bary + self.uv2.1 * w_bary;

        Intersect::new(point, normal, t, self.material.clone(), uv_u, uv_v)
    }
}

/// Representa una malla completa cargada desde un archivo OBJ
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    /// Carga una malla desde un archivo .obj
    pub fn from_obj(
        path: &str,
        material: Material,
        position: Vector3,
        scale: f32,
    ) -> Result<Self, String> {
        let load_result = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );

        let (models, _materials) = load_result.map_err(|e| format!("Error cargando OBJ: {}", e))?;

        let mut triangles = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            
            // Verificar que tengamos índices y posiciones
            if mesh.indices.is_empty() || mesh.positions.is_empty() {
                continue;
            }

            // Procesar cada triángulo
            for face_idx in 0..(mesh.indices.len() / 3) {
                let i0 = mesh.indices[face_idx * 3] as usize;
                let i1 = mesh.indices[face_idx * 3 + 1] as usize;
                let i2 = mesh.indices[face_idx * 3 + 2] as usize;

                // Obtener vértices
                let v0 = Vector3::new(
                    mesh.positions[i0 * 3] * scale + position.x,
                    mesh.positions[i0 * 3 + 1] * scale + position.y,
                    mesh.positions[i0 * 3 + 2] * scale + position.z,
                );
                let v1 = Vector3::new(
                    mesh.positions[i1 * 3] * scale + position.x,
                    mesh.positions[i1 * 3 + 1] * scale + position.y,
                    mesh.positions[i1 * 3 + 2] * scale + position.z,
                );
                let v2 = Vector3::new(
                    mesh.positions[i2 * 3] * scale + position.x,
                    mesh.positions[i2 * 3 + 1] * scale + position.y,
                    mesh.positions[i2 * 3 + 2] * scale + position.z,
                );

                // Obtener normales (o calcularlas si no existen)
                let (n0, n1, n2) = if !mesh.normals.is_empty() {
                    (
                        Vector3::new(
                            mesh.normals[i0 * 3],
                            mesh.normals[i0 * 3 + 1],
                            mesh.normals[i0 * 3 + 2],
                        ).normalized(),
                        Vector3::new(
                            mesh.normals[i1 * 3],
                            mesh.normals[i1 * 3 + 1],
                            mesh.normals[i1 * 3 + 2],
                        ).normalized(),
                        Vector3::new(
                            mesh.normals[i2 * 3],
                            mesh.normals[i2 * 3 + 1],
                            mesh.normals[i2 * 3 + 2],
                        ).normalized(),
                    )
                } else {
                    // Calcular normal del triángulo
                    let normal = (v1 - v0).cross(v2 - v0).normalized();
                    (normal, normal, normal)
                };

                // Obtener coordenadas UV (o usar valores por defecto)
                let (uv0, uv1, uv2) = if !mesh.texcoords.is_empty() {
                    (
                        (mesh.texcoords[i0 * 2], 1.0 - mesh.texcoords[i0 * 2 + 1]),
                        (mesh.texcoords[i1 * 2], 1.0 - mesh.texcoords[i1 * 2 + 1]),
                        (mesh.texcoords[i2 * 2], 1.0 - mesh.texcoords[i2 * 2 + 1]),
                    )
                } else {
                    ((0.0, 0.0), (1.0, 0.0), (0.5, 1.0))
                };

                triangles.push(Triangle {
                    v0, v1, v2,
                    n0, n1, n2,
                    uv0, uv1, uv2,
                    material: material.clone(),
                });
            }
        }

        println!("✅ Malla cargada: {} triángulos desde {}", triangles.len(), path);
        Ok(Mesh { triangles })
    }

    /// Convierte la malla en objetos individuales para el ray tracer
    pub fn to_objects(&self) -> Vec<Arc<dyn RayIntersect + Send + Sync>> {
        self.triangles
            .iter()
            .map(|tri| Arc::new(tri.clone()) as Arc<dyn RayIntersect + Send + Sync>)
            .collect()
    }
}

// Necesitamos implementar Clone para Triangle
impl Clone for Triangle {
    fn clone(&self) -> Self {
        Triangle {
            v0: self.v0,
            v1: self.v1,
            v2: self.v2,
            n0: self.n0,
            n1: self.n1,
            n2: self.n2,
            uv0: self.uv0,
            uv1: self.uv1,
            uv2: self.uv2,
            material: self.material.clone(),
        }
    }
}