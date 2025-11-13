#!/usr/bin/env python3
"""
Generador de texturas de ejemplo para el Ray Tracer
Crea texturas 16x16 básicas en PNG
Requiere: pip install pillow
"""

from PIL import Image, ImageDraw
import os
import random

def ensure_directory():
    """Crea el directorio de texturas si no existe"""
    os.makedirs("assets/textures", exist_ok=True)
    print("✅ Directorio assets/textures/ creado/verificado")

def add_noise(color, amount=10):
    """Añade ruido aleatorio a un color"""
    r, g, b = color
    noise = random.randint(-amount, amount)
    return (
        max(0, min(255, r + noise)),
        max(0, min(255, g + noise)),
        max(0, min(255, b + noise))
    )

def generate_grass_top():
    """Genera textura de césped (vista superior)"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (50, 180, 50)
    
    for y in range(size):
        for x in range(size):
            # Añadir variación de ruido
            color = add_noise(base_color, 20)
            pixels[x, y] = color
    
    img.save("assets/textures/grass_top.png")
    print("✅ grass_top.png generado")

def generate_grass_side():
    """Genera textura de césped lateral (con tierra abajo)"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    grass_color = (50, 180, 50)
    dirt_color = (130, 80, 40)
    
    for y in range(size):
        for x in range(size):
            if y < 4:  # Top 25% is grass
                color = add_noise(grass_color, 15)
            else:  # Bottom 75% is dirt
                color = add_noise(dirt_color, 20)
            pixels[x, y] = color
    
    img.save("assets/textures/grass_side.png")
    print("✅ grass_side.png generado")

def generate_dirt():
    """Genera textura de tierra"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (130, 80, 40)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 25)
            pixels[x, y] = color
    
    img.save("assets/textures/dirt.png")
    print("✅ dirt.png generado")

def generate_stone():
    """Genera textura de piedra"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (100, 100, 100)
    
    for y in range(size):
        for x in range(size):
            # Más variación para simular textura de piedra
            color = add_noise(base_color, 30)
            
            # Algunos píxeles más oscuros para grietas
            if random.random() < 0.1:
                color = (max(0, color[0] - 40), max(0, color[1] - 40), max(0, color[2] - 40))
            
            pixels[x, y] = color
    
    img.save("assets/textures/stone.png")
    print("✅ stone.png generado")

def generate_wood():
    """Genera textura de madera con anillos"""
    size = 16
    img = Image.new('RGB', (size, size))
    draw = ImageDraw.Draw(img)
    pixels = img.load()
    
    # Base color
    base_color = (80, 50, 20)
    for y in range(size):
        for x in range(size):
            pixels[x, y] = add_noise(base_color, 15)
    
    # Añadir anillos concéntricos
    center = size // 2
    for radius in [2, 4, 6]:
        color = (max(0, base_color[0] - 20), max(0, base_color[1] - 15), max(0, base_color[2] - 10))
        draw.ellipse([center - radius, center - radius, center + radius, center + radius], 
                     outline=color, width=1)
    
    img.save("assets/textures/wood.png")
    print("✅ wood.png generado")

def generate_leaves():
    """Genera textura de hojas"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (40, 120, 40)
    
    for y in range(size):
        for x in range(size):
            # Variación más grande para simular hojas
            color = add_noise(base_color, 35)
            
            # Algunos píxeles más oscuros
            if random.random() < 0.15:
                color = (max(0, color[0] - 20), max(0, color[1] - 20), max(0, color[2] - 20))
            
            pixels[x, y] = color
    
    img.save("assets/textures/leaves.png")
    print("✅ leaves.png generado")

def generate_water():
    """Genera textura de agua animada"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (30, 80, 200)
    
    for y in range(size):
        for x in range(size):
            # Patrón de olas
            wave = ((x + y) % 4) * 8
            color = (
                min(255, base_color[0] + wave),
                min(255, base_color[1] + wave),
                min(255, base_color[2] + wave)
            )
            pixels[x, y] = color
    
    img.save("assets/textures/water.png")
    print("✅ water.png generado")

def generate_lava():
    """Genera textura de lava"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    for y in range(size):
        for x in range(size):
            # Gradiente de naranja a rojo
            t = random.random()
            if t < 0.3:
                color = (255, random.randint(180, 220), random.randint(0, 30))
            elif t < 0.7:
                color = (255, random.randint(100, 150), random.randint(0, 20))
            else:
                color = (random.randint(200, 255), random.randint(50, 100), 0)
            
            pixels[x, y] = color
    
    img.save("assets/textures/lava.png")
    print("✅ lava.png generado")

def generate_netherrack():
    """Genera textura de netherrack"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (150, 50, 50)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 40)
            pixels[x, y] = color
    
    img.save("assets/textures/netherrack.png")
    print("✅ netherrack.png generado")

def generate_nether_brick():
    """Genera textura de ladrillos del Nether"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    brick_color = (50, 15, 15)
    mortar_color = (20, 10, 10)
    
    for y in range(size):
        for x in range(size):
            # Patrón de ladrillos
            if x % 8 == 0 or y % 8 == 0:
                pixels[x, y] = add_noise(mortar_color, 5)
            else:
                pixels[x, y] = add_noise(brick_color, 10)
    
    img.save("assets/textures/nether_brick.png")
    print("✅ nether_brick.png generado")

def generate_soul_sand():
    """Genera textura de arena de almas"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (70, 50, 35)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 20)
            
            # Algunos píxeles más oscuros para "almas"
            if random.random() < 0.1:
                color = (max(0, color[0] - 30), max(0, color[1] - 30), max(0, color[2] - 30))
            
            pixels[x, y] = color
    
    img.save("assets/textures/soul_sand.png")
    print("✅ soul_sand.png generado")

def generate_glowstone():
    """Genera textura de piedra luminosa"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (255, 220, 100)
    
    for y in range(size):
        for x in range(size):
            # Variación de brillo
            brightness = random.randint(-20, 20)
            color = (
                min(255, base_color[0] + brightness),
                min(255, base_color[1] + brightness),
                min(255, max(0, base_color[2] + brightness))
            )
            pixels[x, y] = color
    
    img.save("assets/textures/glowstone.png")
    print("✅ glowstone.png generado")

def generate_brick():
    """Genera textura de ladrillos normales"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    brick_color = (150, 80, 60)
    mortar_color = (180, 180, 180)
    
    for y in range(size):
        for x in range(size):
            # Patrón de ladrillos con offset cada dos filas
            offset = (y // 4) % 2 * 4
            if (x + offset) % 8 == 0 or y % 4 == 0:
                pixels[x, y] = add_noise(mortar_color, 10)
            else:
                pixels[x, y] = add_noise(brick_color, 15)
    
    img.save("assets/textures/brick.png")
    print("✅ brick.png generado (bonus)")

def generate_sand():
    """Genera textura de arena"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (210, 180, 140)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 15)
            pixels[x, y] = color
    
    img.save("assets/textures/sand.png")
    print("✅ sand.png generado (bonus)")

def main():
    """Genera todas las texturas"""
    print("🎨 Generador de Texturas para Ray Tracer")
    print("=" * 50)
    
    ensure_directory()
    print("\n📦 Generando texturas 16x16...\n")
    
    # Texturas básicas
    generate_grass_top()
    generate_grass_side()
    generate_dirt()
    generate_stone()
    generate_wood()
    generate_leaves()
    generate_water()
    generate_lava()
    
    # Texturas del Nether
    generate_netherrack()
    generate_nether_brick()
    generate_soul_sand()
    generate_glowstone()
    
    # Texturas bonus
    generate_brick()
    generate_sand()
    
    print("\n" + "=" * 50)
    print("✨ ¡Todas las texturas generadas exitosamente!")
    print(f"📁 Las encontrarás en: assets/textures/")
    print("\n💡 Tip: Puedes editar estas texturas con cualquier editor")
    print("   de imágenes para personalizarlas a tu gusto")

if __name__ == "__main__":
    try:
        main()
    except ImportError:
        print("❌ Error: PIL (Pillow) no está instalado")
        print("Instálalo con: pip install pillow")
    except Exception as e:
        print(f"❌ Error: {e}")