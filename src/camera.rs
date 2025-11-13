use raylib::prelude::*;

/// Cámara orbital 3D que mantiene su posición y orientación en el espacio
pub struct Camera {
    /// Posición de la cámara en coordenadas del mundo
    pub eye: Vector3,
    
    /// Punto hacia el que mira la cámara
    pub center: Vector3,
    
    /// Vector "arriba" de la cámara (se ortonormaliza automáticamente)
    pub up: Vector3,
    
    /// Dirección hacia donde mira la cámara (calculado de eye->center)
    pub forward: Vector3,
    
    /// Dirección hacia la derecha (perpendicular a forward y up)
    pub right: Vector3,
}

impl Camera {
    /// Crea una nueva cámara y calcula su orientación inicial
    pub fn new(eye: Vector3, center: Vector3, up: Vector3) -> Self {
        let mut camera = Camera {
            eye,
            center,
            up,
            forward: Vector3::zero(),
            right: Vector3::zero(),
        };
        camera.update_basis_vectors();
        camera
    }

    /// Recalcula los vectores de base ortonormal de la cámara
    fn update_basis_vectors(&mut self) {
        // 1. Calcular dirección forward (de eye hacia center)
        self.forward = (self.center - self.eye).normalized();
        
        // 2. Calcular dirección right usando producto cruz
        // forward × up da un vector perpendicular a ambos (apuntando a la derecha)
        self.right = self.forward.cross(self.up).normalized();
        
        // 3. Recalcular up para asegurar ortogonalidad perfecta
        // right × forward da un vector perpendicular a ambos
        self.up = self.right.cross(self.forward);
    }

    /// Rota la cámara alrededor del punto central (movimiento orbital)
    /// 
    /// # Argumentos
    /// * `yaw` - Rotación horizontal (alrededor del eje Y)
    /// * `pitch` - Rotación vertical (elevación)
    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        // 1. Obtener posición relativa al centro
        let relative_pos = self.eye - self.center;
        
        // 2. Convertir a coordenadas esféricas
        let radius = relative_pos.length();
        let current_yaw = relative_pos.z.atan2(relative_pos.x);
        let current_pitch = (relative_pos.y / radius).asin();
        
        // 3. Aplicar rotaciones
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(-1.5, 1.5);
        
        // 4. Convertir de vuelta a coordenadas cartesianas
        let cos_pitch = new_pitch.cos();
        let new_relative_pos = Vector3::new(
            radius * cos_pitch * new_yaw.cos(),
            radius * new_pitch.sin(),
            radius * cos_pitch * new_yaw.sin(),
        );
        
        // 5. Actualizar posición en coordenadas del mundo
        self.eye = self.center + new_relative_pos;
        
        // 6. Recalcular vectores de base
        self.update_basis_vectors();
    }

    /// Acerca o aleja la cámara del punto central
    /// 
    /// # Argumentos
    /// * `amount` - Cantidad de zoom (positivo = acercar, negativo = alejar)
    pub fn zoom(&mut self, amount: f32) {
        let forward = (self.center - self.eye).normalized();
        self.eye = self.eye + forward * amount;
        self.update_basis_vectors();
    }

    /// Transforma un vector del espacio de cámara al espacio del mundo
    /// 
    /// # Argumentos
    /// * `v` - Vector en coordenadas de cámara
    /// 
    /// # Retorna
    /// El mismo vector expresado en coordenadas del mundo
    pub fn basis_change(&self, v: &Vector3) -> Vector3 {
        // En espacio de cámara:
        // - X apunta a la derecha
        // - Y apunta arriba
        // - Z apunta hacia atrás (la cámara mira en -Z)
        Vector3::new(
            v.x * self.right.x + v.y * self.up.x - v.z * self.forward.x,
            v.x * self.right.y + v.y * self.up.y - v.z * self.forward.y,
            v.x * self.right.z + v.y * self.up.z - v.z * self.forward.z,
        )
    }
}