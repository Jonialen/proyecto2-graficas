use raylib::prelude::*;

/// Buffer de imagen que almacena el resultado del renderizado
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    /// Crea un nuevo framebuffer con las dimensiones especificadas
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    /// Limpia el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width as i32,
            self.height as i32,
            self.background_color,
        );
    }

    /// Establece un pixel en la posición especificada con el color actual
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    /// Establece el color de fondo del framebuffer
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Establece el color actual para dibujar pixels
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Exporta el framebuffer a un archivo de imagen
    /// Útil para guardar capturas de la escena renderizada
    #[allow(dead_code)]
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    /// Muestra el framebuffer en la ventana de raylib
    /// Convierte la imagen en una textura y la dibuja
    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}