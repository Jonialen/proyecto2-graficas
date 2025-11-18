# Ray Tracer en Rust

Este es un proyecto de ray tracing escrito en Rust, inspirado en el estilo visual de Minecraft. Es capaz de renderizar escenas complejas con iluminación, sombras, reflejos, refracción y texturas, incluyendo texturas animadas.

## Características

-   **Motor de Ray Tracing:** Implementado desde cero en Rust.
-   **Bounding Volume Hierarchy (BVH):** Para la aceleración de la intersección de rayos.
-   **Iluminación y Sombras:** Soporte para múltiples fuentes de luz y sombras realistas.
-   **Materiales:** Sistema de materiales con soporte para reflejos, refracción y texturas.
-   **Texturas:** Carga dinámica de texturas desde disco, con un sistema de fallback a texturas procedurales.
-   **Texturas Animadas:** Soporte para texturas animadas para efectos de agua, lava y portales.
-   **Generador de Texturas:** Un script en Python (`app.py`) para generar todas las texturas del proyecto.
-   **Exportación de Texturas:** Una función para exportar todas las texturas cargadas a un directorio.
-   **Escenas Múltiples:** Varias escenas predefinidas para explorar las capacidades del motor.
-   **Ciclo de Día y Noche:** Simulación de un ciclo de día y noche con cambios en la iluminación y el color del cielo.

## Cómo Empezar

Sigue estas instrucciones para tener una copia del proyecto funcionando en tu máquina local para propósitos de desarrollo y pruebas.

### Prerrequisitos

Necesitarás tener instalado lo siguiente en tu sistema:

-   [Rust](https://www.rust-lang.org/tools/install) (incluyendo `cargo`)
-   [Python 3](https://www.python.org/downloads/)
-   La librería de Python `Pillow`:
    ```sh
    pip install pillow
    ```

### Instalación

1.  **Clona el repositorio:**
    ```sh
    git clone https://github.com/Jonialen/proyecto2-graficas.git
    cd proyecto2-graficas
    ```

2.  **Genera las texturas:**
    El proyecto incluye un script de Python para generar todas las texturas necesarias.
    ```sh
    python3 app.py
    ```
    Esto creará las texturas en el directorio `assets/textures`.

3.  **Construye y ejecuta el proyecto:**
    Puedes construir y ejecutar el proyecto usando `cargo`.
    ```sh
    cargo run --release
    ```
    La opción `--release` es recomendada para un rendimiento óptimo.

## Uso

Una vez que la aplicación esté en ejecución, verás una ventana con la escena renderizada. Puedes interactuar con la escena usando los controles del teclado.

### Controles

| Tecla         | Acción                        |
| ------------- | ----------------------------- |
| `1-9`, `0`, `-`, `=` | Cambiar de escena             |
| `←` `→`       | Rotar la cámara horizontalmente |
| `↑` `↓`         | Rotar la cámara verticalmente   |
| `W` `S`         | Hacer zoom (acercar/alejar)   |
| `R`           | Resetear la cámara            |
| `P`           | Pausar/reanudar el ciclo de día y noche |
| `[`           | Adelantar el tiempo           |
| `]`           | Retroceder el tiempo          |
| `E`           | Exportar todas las texturas a `assets/textures_exported` |
| `ESC`         | Salir de la aplicación        |

### Escenas Disponibles

Puedes cambiar entre las siguientes escenas usando las teclas numéricas:

1.  **Isla Flotante Básica**
2.  **Isla con Cascadas**
3.  **Isla con Puente Portal**
4.  **Castillo Medieval**
5.  **Casa con Jardín**
6.  **Escena Simple**
7.  **Aldea Medieval**
8.  **Bosque Encantado**
9.  **Archipiélago Masivo**
10. **Templo Antiguo**
11. **Cañón con Río**
12. **Portal Dimensional**

## Construido Con

-   [Rust](https://www.rust-lang.org/) - El lenguaje de programación principal.
-   [Raylib](https://www.raylib.com/) - Para la creación de la ventana y la gestión de la entrada.
-   [Rayon](https://github.com/rayon-rs/rayon) - Para el paralelismo de datos.
-   [image-rs](https://github.com/image-rs/image) - Para la carga y guardado de imágenes.
-   [tobj](https://github.com/Twinklebear/tobj) - Para la carga de modelos 3D en formato `.obj`.
-   [Pillow](https://python-pillow.org/) - Para la generación de texturas en el script de Python.
