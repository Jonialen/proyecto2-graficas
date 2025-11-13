use raylib::prelude::*;

/// Representa una fuente de luz puntual en la escena
pub struct Light {
    /// Posición de la luz en el espacio 3D
    pub position: Vector3,
    
    /// Color de la luz
    pub color: Color,
    
    /// Intensidad de la luz (multiplicador de brillo)
    pub intensity: f32,
}

impl Light {
    /// Crea una nueva fuente de luz
    /// 
    /// # Argumentos
    /// * `position` - Posición de la luz en coordenadas del mundo
    /// * `color` - Color RGB de la luz
    /// * `intensity` - Intensidad/brillo de la luz (típicamente 0.5 - 5.0)
    pub fn new(position: Vector3, color: Color, intensity: f32) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }
}