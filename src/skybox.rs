use raylib::prelude::Vector3;

/// Configuración de colores para diferentes momentos del día
pub struct SkyColors {
    pub zenith: Vector3,
    pub horizon: Vector3,
    pub cloud_color: Vector3,
}

/// Genera colores del cielo según la hora del día
pub fn get_sky_colors(time_of_day: f32) -> SkyColors {
    if time_of_day < 0.2 {
        // Noche (0.0 - 0.2)
        let t = time_of_day / 0.2;
        let deep_night = Vector3::new(0.02, 0.02, 0.1);
        let late_night = Vector3::new(0.05, 0.05, 0.15);
        let horizon = deep_night * (1.0 - t) + late_night * t;
        
        SkyColors {
            zenith: Vector3::new(0.01, 0.01, 0.08),
            horizon,
            cloud_color: Vector3::new(0.15, 0.15, 0.25),
        }
    } else if time_of_day < 0.3 {
        // Amanecer - Morado/Rosa
        let t = (time_of_day - 0.2) / 0.1;
        SkyColors {
            zenith: Vector3::new(0.05, 0.05, 0.15) * (1.0 - t) 
                  + Vector3::new(0.3, 0.15, 0.4) * t,
            horizon: Vector3::new(0.05, 0.05, 0.15) * (1.0 - t) 
                   + Vector3::new(0.9, 0.4, 0.6) * t,
            cloud_color: Vector3::new(0.7 + t * 0.2, 0.3 + t * 0.4, 0.5 + t * 0.3),
        }
    } else if time_of_day < 0.35 {
        // Amanecer - Naranja/Dorado
        let t = (time_of_day - 0.3) / 0.05;
        SkyColors {
            zenith: Vector3::new(0.3, 0.15, 0.4) * (1.0 - t) 
                  + Vector3::new(0.4, 0.5, 0.9) * t,
            horizon: Vector3::new(0.9, 0.4, 0.6) * (1.0 - t) 
                   + Vector3::new(1.0, 0.6, 0.3) * t,
            cloud_color: Vector3::new(1.0, 0.8 + t * 0.15, 0.6 + t * 0.3),
        }
    } else if time_of_day < 0.45 {
        // Mañana temprana
        let t = (time_of_day - 0.35) / 0.1;
        SkyColors {
            zenith: Vector3::new(0.4, 0.5, 0.9) * (1.0 - t) 
                  + Vector3::new(0.3, 0.5, 1.0) * t,
            horizon: Vector3::new(1.0, 0.6, 0.3) * (1.0 - t) 
                   + Vector3::new(0.6, 0.8, 1.0) * t,
            cloud_color: Vector3::new(1.0, 0.95 + t * 0.05, 0.9 + t * 0.1),
        }
    } else if time_of_day < 0.65 {
        // Día brillante
        SkyColors {
            zenith: Vector3::new(0.2, 0.4, 0.95),
            horizon: Vector3::new(0.5, 0.7, 1.0),
            cloud_color: Vector3::new(1.0, 1.0, 1.0),
        }
    } else if time_of_day < 0.7 {
        // Tarde
        let t = (time_of_day - 0.65) / 0.05;
        SkyColors {
            zenith: Vector3::new(0.2, 0.4, 0.95) * (1.0 - t) 
                  + Vector3::new(0.4, 0.5, 0.9) * t,
            horizon: Vector3::new(0.5, 0.7, 1.0) * (1.0 - t) 
                   + Vector3::new(0.7, 0.7, 0.9) * t,
            cloud_color: Vector3::new(1.0, 1.0 - t * 0.05, 1.0 - t * 0.1),
        }
    } else if time_of_day < 0.75 {
        // Atardecer - Naranja/Dorado
        let t = (time_of_day - 0.7) / 0.05;
        SkyColors {
            zenith: Vector3::new(0.4, 0.5, 0.9) * (1.0 - t) 
                  + Vector3::new(0.6, 0.4, 0.7) * t,
            horizon: Vector3::new(0.7, 0.7, 0.9) * (1.0 - t) 
                   + Vector3::new(1.0, 0.5, 0.2) * t,
            cloud_color: Vector3::new(1.0, 0.8 - t * 0.2, 0.6 - t * 0.3),
        }
    } else if time_of_day < 0.8 {
        // Atardecer - Rojo/Púrpura
        let t = (time_of_day - 0.75) / 0.05;
        SkyColors {
            zenith: Vector3::new(0.6, 0.4, 0.7) * (1.0 - t) 
                  + Vector3::new(0.4, 0.2, 0.5) * t,
            horizon: Vector3::new(1.0, 0.5, 0.2) * (1.0 - t) 
                   + Vector3::new(0.9, 0.3, 0.4) * t,
            cloud_color: Vector3::new(0.9 - t * 0.2, 0.4 - t * 0.1, 0.5),
        }
    } else if time_of_day < 0.9 {
        // Crepúsculo
        let t = (time_of_day - 0.8) / 0.1;
        SkyColors {
            zenith: Vector3::new(0.4, 0.2, 0.5) * (1.0 - t) 
                  + Vector3::new(0.1, 0.1, 0.25) * t,
            horizon: Vector3::new(0.9, 0.3, 0.4) * (1.0 - t) 
                   + Vector3::new(0.3, 0.15, 0.3) * t,
            cloud_color: Vector3::new(0.5 - t * 0.2, 0.3 - t * 0.15, 0.5 - t * 0.2),
        }
    } else {
        // Noche temprana
        let t = (time_of_day - 0.9) / 0.1;
        SkyColors {
            zenith: Vector3::new(0.1, 0.1, 0.25) * (1.0 - t) 
                  + Vector3::new(0.02, 0.02, 0.1) * t,
            horizon: Vector3::new(0.3, 0.15, 0.3) * (1.0 - t) 
                   + Vector3::new(0.05, 0.05, 0.15) * t,
            cloud_color: Vector3::new(0.3, 0.3, 0.5),
        }
    }
}

/// Genera ruido procedural para nubes volumétricas
#[inline]
fn cloud_noise(x: f32, y: f32, z: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        let sample_x = x * frequency;
        let sample_y = y * frequency;
        let sample_z = z * frequency;

        // Ruido basado en funciones trigonométricas
        let noise = (sample_x.sin() * sample_y.cos() + 
                     (sample_x * 1.3 + sample_z).cos() * (sample_y * 0.7).sin() +
                     (sample_z * 1.7).sin() * (sample_x * 0.5).cos()) / 3.0;

        value += noise * amplitude;
        max_value += amplitude;

        amplitude *= 0.5;
        frequency *= 2.0;
    }

    (value / max_value) * 0.5 + 0.5
}

/// Genera nubes volumétricas realistas
fn generate_clouds(dir: Vector3, time_of_day: f32, cloud_color: Vector3) -> (f32, Vector3) {
    if dir.y <= 0.1 {
        return (0.0, cloud_color);
    }

    // Diferentes capas de nubes
    let altitude = (dir.y - 0.1).clamp(0.0, 0.8);
    let altitude_factor = (1.0 - ((altitude - 0.4) / 0.3).abs().min(1.0)).powf(2.0);

    if altitude_factor < 0.1 {
        return (0.0, cloud_color);
    }

    // Coordenadas normalizadas para sampling
    let cloud_scale = 3.0;
    let x = dir.x / (dir.y + 0.1) * cloud_scale;
    let z = dir.z / (dir.y + 0.1) * cloud_scale;

    // Múltiples capas de ruido para volumen
    let large_clouds = cloud_noise(x, z, time_of_day * 0.1, 3);
    let medium_detail = cloud_noise(x * 2.5, z * 2.5, time_of_day * 0.15, 2);
    let fine_detail = cloud_noise(x * 7.0, z * 7.0, time_of_day * 0.2, 1);

    // Combinar capas
    let cloud_density = large_clouds * 0.6 + medium_detail * 0.25 + fine_detail * 0.15;

    // Threshold para formar nubes definidas
    let threshold = 0.5;
    let cloud_alpha = if cloud_density > threshold {
        ((cloud_density - threshold) / (1.0 - threshold)).powf(1.5) * altitude_factor
    } else {
        0.0
    };

    // Variación de color en las nubes
    let cloud_variation = fine_detail * 0.15;
    let varied_cloud_color = cloud_color * (0.85 + cloud_variation);

    (cloud_alpha.clamp(0.0, 0.7), varied_cloud_color)
}

/// Genera estrellas en el cielo nocturno
fn generate_stars(dir: Vector3, time_of_day: f32) -> Vector3 {
    if time_of_day > 0.2 && time_of_day < 0.85 {
        return Vector3::zero();
    }

    if dir.y <= 0.0 {
        return Vector3::zero();
    }

    // Patrón de estrellas usando múltiples frecuencias
    let star_pattern = ((dir.x * 50.0).sin() * (dir.y * 50.0).cos() * (dir.z * 50.0).sin()).abs();
    let star_pattern2 = ((dir.x * 80.0 + 10.0).cos() * (dir.y * 70.0).sin() * (dir.z * 90.0).cos()).abs();
    
    let combined_pattern = (star_pattern + star_pattern2) * 0.5;
    
    let star_threshold = 0.998;
    
    if combined_pattern > star_threshold {
        let star_brightness = ((combined_pattern - star_threshold) / (1.0 - star_threshold)).powf(2.0);
        
        // Intensidad según hora de la noche
        let night_intensity = if time_of_day < 0.2 {
            (0.2 - time_of_day) / 0.2
        } else {
            (time_of_day - 0.85) / 0.15
        };
        
        // Colores variados para las estrellas
        let star_color = if star_pattern > star_pattern2 {
            Vector3::new(1.0, 1.0, 0.9) // Blanco amarillento
        } else {
            Vector3::new(0.9, 0.95, 1.0) // Blanco azulado
        };
        
        return star_color * star_brightness * night_intensity * 0.6;
    }

    Vector3::zero()
}

/// Skybox del Overworld con nubes y estrellas
pub fn overworld_sky(dir: Vector3, time_of_day: f32) -> Vector3 {
    let d = dir.normalized();
    let colors = get_sky_colors(time_of_day);

    // Gradiente vertical base
    let vertical_gradient = if d.y >= 0.0 {
        let t = d.y.powf(0.6);
        colors.horizon * (1.0 - t) + colors.zenith * t
    } else {
        let t = (-d.y).powf(0.8);
        let below_horizon = colors.horizon * 0.6;
        colors.horizon * (1.0 - t) + below_horizon * t
    };

    // Agregar nubes volumétricas
    let (cloud_alpha, cloud_color) = generate_clouds(d, time_of_day, colors.cloud_color);
    let sky_with_clouds = vertical_gradient * (1.0 - cloud_alpha) + cloud_color * cloud_alpha;

    // Agregar estrellas en la noche
    let stars = generate_stars(d, time_of_day);
    sky_with_clouds + stars
}

/// Genera ruido para efectos del Nether
#[inline]
fn nether_noise(x: f32, y: f32, z: f32) -> f32 {
    let n1 = (x * 3.0).sin() * (y * 2.5).cos();
    let n2 = (x * 1.7 + z * 2.3).cos() * (y * 1.9).sin();
    let n3 = (z * 2.9).sin() * (x * 1.3).cos();
    (n1 + n2 + n3) / 3.0
}

/// Genera partículas de ceniza flotante
fn ash_particles(dir: Vector3, time: f32) -> f32 {
    // Múltiples capas de partículas a diferentes velocidades
    let particle1 = ((dir.x * 100.0 + time * 2.0).sin() * 
                     (dir.y * 80.0 - time * 1.5).cos() * 
                     (dir.z * 120.0 + time * 2.5).sin()).abs();
    
    let particle2 = ((dir.x * 150.0 - time * 1.8).cos() * 
                     (dir.y * 130.0 + time * 2.2).sin() * 
                     (dir.z * 90.0 - time * 1.3).cos()).abs();
    
    let combined = (particle1 + particle2) * 0.5;
    
    // Threshold más alto para partículas dispersas
    if combined > 0.995 {
        let intensity = ((combined - 0.995) / 0.005).min(1.0);
        return intensity * 0.4;
    }
    
    0.0
}

/// Skybox del Nether con efectos especiales
pub fn nether_sky(dir: Vector3, time: f32) -> Vector3{
    let d = dir.normalized();
    
    // Techo rocoso con grietas de lava
    if d.y > 0.3 {
        let base_rock = Vector3::new(0.25, 0.05, 0.05);
        
        // Patrón de grietas con lava
        let crack_pattern = nether_noise(d.x * 8.0, d.z * 8.0, time * 0.1);
        let lava_cracks = if crack_pattern > 0.3 {
            let glow = ((crack_pattern - 0.3) / 0.7).powf(2.0);
            Vector3::new(1.0, 0.3, 0.0) * glow * 0.5
        } else {
            Vector3::zero()
        };
        
        // Variación de textura en la roca
        let rock_variation = (nether_noise(d.x * 15.0, d.z * 15.0, 0.0) * 0.5 + 0.5) * 0.2;
        
        return base_rock * (0.8 + rock_variation) + lava_cracks;
    }
    
    // Niebla del Nether con gradiente
    if d.y > -0.3 {
        let t = (d.y + 0.3) / 0.6;
        
        // Color base de la niebla
        let top_fog = Vector3::new(0.3, 0.05, 0.05);
        let bottom_fog = Vector3::new(0.2, 0.02, 0.02);
        let base_color = top_fog * t + bottom_fog * (1.0 - t);
        
        // Efecto de distorsión por calor
        let heat_distortion = nether_noise(
            d.x * 5.0 + time * 0.5,
            d.y * 4.0,
            d.z * 5.0 + time * 0.7
        ) * 0.5 + 0.5;
        
        // Vórtices de calor
        let heat_shimmer = Vector3::new(0.15, 0.03, 0.0) * heat_distortion * 0.3;
        
        // Partículas de ceniza
        let ash = ash_particles(d, time);
        let ash_color = Vector3::new(0.4, 0.35, 0.3) * ash;
        
        return base_color + heat_shimmer + ash_color;
    }
    
    // Parte inferior - resplandor de lava
    let t = (-d.y - 0.3) / 0.7;
    
    // Color base del resplandor
    let lava_glow = Vector3::new(0.6, 0.15, 0.0);
    let deep_fog = Vector3::new(0.15, 0.01, 0.01);
    let base_color = deep_fog * (1.0 - t) + lava_glow * t;
    
    // Pulso de lava
    let pulse = (time * 2.0).sin() * 0.5 + 0.5;
    let lava_pulse = Vector3::new(0.3, 0.05, 0.0) * pulse * t * 0.4;
    
    // Burbujas de lava (solo en la parte más baja)
    let bubbles = if t > 0.7 {
        let bubble_noise = nether_noise(d.x * 20.0 + time, d.z * 20.0 + time * 1.3, 0.0);
        if bubble_noise > 0.6 {
            Vector3::new(1.0, 0.4, 0.1) * ((bubble_noise - 0.6) / 0.4) * 0.3
        } else {
            Vector3::zero()
        }
    } else {
        Vector3::zero()
    };
    
    base_color + lava_pulse + bubbles
}

/// Función principal que selecciona el skybox apropiado
pub fn sky_color(dir: Vector3, is_nether: bool, time_of_day: f32, time: f32) -> Vector3 {
    if is_nether {
        nether_sky(dir, time)
    } else {
        overworld_sky(dir, time_of_day)
    }
}