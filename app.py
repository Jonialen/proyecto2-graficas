#!/usr/bin/env python3
"""
Generador de texturas completo para el Ray Tracer
Crea texturas 16x16 en PNG (estáticas y animadas)
Requiere: pip install pillow
"""

from PIL import Image, ImageDraw, ImageFilter
import os
import random
import math

def ensure_directory():
    """Crea el directorio de texturas si no existe"""
    os.makedirs("assets/textures", exist_ok=True)
    print("Directorio assets/textures/ creado/verificado")

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
            color = add_noise(base_color, 20)
            # Añadir algunos píxeles más oscuros para variación
            if random.random() < 0.15:
                color = tuple(max(0, c - 30) for c in color)
            pixels[x, y] = color
    
    img.save("assets/textures/grass_top.png")
    print("grass_top.png")

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
    print("grass_side.png")

def generate_dirt():
    """Genera textura de tierra"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (130, 80, 40)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 25)
            # Añadir pequeñas piedras ocasionales
            if random.random() < 0.05:
                color = (90, 90, 90)
            pixels[x, y] = color
    
    img.save("assets/textures/dirt.png")
    print("dirt.png")

def generate_stone():
    """Genera textura de piedra"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (100, 100, 100)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 30)
            
            # Grietas y variación
            if random.random() < 0.1:
                color = tuple(max(0, c - 40) for c in color)
            elif random.random() < 0.05:
                color = tuple(min(255, c + 20) for c in color)
            
            pixels[x, y] = color
    
    img.save("assets/textures/stone.png")
    print("stone.png")

def generate_wood():
    """Genera textura de madera con anillos"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (80, 50, 20)
    center = size // 2
    
    for y in range(size):
        for x in range(size):
            # Distancia al centro para anillos
            dist = math.sqrt((x - center)**2 + (y - center)**2)
            ring = int(dist * 2) % 2
            
            if ring == 0:
                color = add_noise(base_color, 10)
            else:
                darker = tuple(max(0, c - 15) for c in base_color)
                color = add_noise(darker, 10)
            
            pixels[x, y] = color
    
    img.save("assets/textures/wood.png")
    print("wood.png")

def generate_leaves():
    """Genera textura de hojas"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (40, 120, 40)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 35)
            
            # Sombras y luces
            if random.random() < 0.15:
                color = tuple(max(0, c - 25) for c in color)
            elif random.random() < 0.1:
                color = tuple(min(255, c + 30) for c in color)
            
            pixels[x, y] = color
    
    img.save("assets/textures/leaves.png")
    print("leaves.png")

def generate_water_animated():
    """Genera 4 frames de agua animada"""
    size = 16
    
    for frame in range(4):
        img = Image.new('RGB', (size, size))
        pixels = img.load()
        
        base_color = (30, 80, 200)
        offset = frame * 2
        
        for y in range(size):
            for x in range(size):
                # Patrón de olas animado
                wave = ((x + offset + y) % 4) * 8
                color = (
                    min(255, base_color[0] + wave),
                    min(255, base_color[1] + wave),
                    min(255, base_color[2] + wave)
                )
                pixels[x, y] = color
        
        img.save(f"assets/textures/water_{frame}.png")
    
    print("water_0.png, water_1.png, water_2.png, water_3.png")

def generate_lava_animated():
    """Genera 4 frames de lava animada"""
    size = 16
    
    for frame in range(4):
        img = Image.new('RGB', (size, size))
        pixels = img.load()
        
        offset = frame * 3
        
        for y in range(size):
            for x in range(size):
                # Flujo de lava animado
                flow = ((x * 3 + y * 7 + offset) % 12) / 12.0
                
                if flow < 0.3:
                    color = (255, 100 + int(flow * 200), int(flow * 50))
                elif flow < 0.7:
                    color = (255, 180 - int(flow * 100), 0)
                else:
                    color = (200 + int(flow * 55), 80, 0)
                
                # Burbujas ocasionales
                if random.random() < 0.05:
                    color = (255, 200, 50)
                
                pixels[x, y] = color
        
        img.save(f"assets/textures/lava_{frame}.png")
    
    print("lava_0.png, lava_1.png, lava_2.png, lava_3.png")

def generate_portal_animated():
    """Genera 6 frames de portal animado"""
    size = 16
    
    for frame in range(6):
        img = Image.new('RGB', (size, size))
        pixels = img.load()
        
        phase = frame * math.pi / 3
        center = size / 2
        
        for y in range(size):
            for x in range(size):
                # Distancia al centro
                dx = x - center
                dy = y - center
                dist = math.sqrt(dx**2 + dy**2)
                angle = math.atan2(dy, dx)
                
                # Espiral psicodélica
                wave = math.sin(dist * 0.8 + phase) * 50
                swirl = math.sin(angle * 3 + phase) * 30
                
                purple = int(128 + wave + swirl)
                magenta = int(0 + wave / 3 + swirl / 2)
                violet = int(200 + wave + swirl)
                
                color = (
                    max(80, min(220, purple)),
                    max(0, min(120, magenta)),
                    max(150, min(255, violet))
                )
                
                pixels[x, y] = color
        
        img.save(f"assets/textures/portal_{frame}.png")
    
    print("portal_0.png ... portal_5.png")

def generate_netherrack():
    """Genera textura de netherrack"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (150, 50, 50)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 40)
            
            # Vetas más oscuras
            if (x + y * 3) % 7 == 0:
                color = tuple(max(0, c - 30) for c in color)
            
            pixels[x, y] = color
    
    img.save("assets/textures/netherrack.png")
    print("netherrack.png")

def generate_nether_brick():
    """Genera textura de ladrillos del Nether"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    brick_color = (50, 15, 15)
    mortar_color = (20, 10, 10)
    
    for y in range(size):
        for x in range(size):
            if x % 8 == 0 or y % 8 == 0:
                pixels[x, y] = add_noise(mortar_color, 5)
            else:
                pixels[x, y] = add_noise(brick_color, 10)
    
    img.save("assets/textures/nether_brick.png")
    print("nether_brick.png")

def generate_soul_sand():
    """Genera textura de arena de almas"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (70, 50, 35)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 20)
            
            # "Caras" de almas ocasionales
            if random.random() < 0.08:
                color = tuple(max(0, c - 35) for c in color)
            
            pixels[x, y] = color
    
    img.save("assets/textures/soul_sand.png")
    print("soul_sand.png")

def generate_glowstone():
    """Genera textura de piedra luminosa"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (255, 220, 100)
    
    for y in range(size):
        for x in range(size):
            brightness = random.randint(-20, 20)
            
            # Cristales más brillantes
            if ((x + y) % 3) == 0:
                brightness += 20
            
            color = (
                min(255, base_color[0] + brightness),
                min(255, base_color[1] + brightness),
                min(255, max(0, base_color[2] + brightness))
            )
            pixels[x, y] = color
    
    img.save("assets/textures/glowstone.png")
    print("glowstone.png")

def generate_diamond():
    """Genera textura de diamante cristalino"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (180, 230, 255)
    center = size / 2
    
    for y in range(size):
        for x in range(size):
            # Facetas del cristal
            dx = abs(x - center)
            dy = abs(y - center)
            facet = int((dx + dy) * 2) % 3
            
            color = add_noise(base_color, 20)
            
            # Brillo en ciertas facetas
            if facet == 0:
                color = tuple(min(255, c + 30) for c in color)
            elif facet == 2:
                color = tuple(max(0, c - 20) for c in color)
            
            # Destellos
            if random.random() < 0.05:
                color = (255, 255, 255)
            
            pixels[x, y] = color
    
    img.save("assets/textures/diamond.png")
    print("diamond.png")

def generate_emerald():
    """Genera textura de esmeralda"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (50, 230, 80)
    center = size / 2
    
    for y in range(size):
        for x in range(size):
            # Facetas hexagonales
            dx = abs(x - center)
            dy = abs(y - center)
            facet = int((dx + dy) * 2) % 3
            
            color = add_noise(base_color, 25)
            
            if facet == 0:
                color = tuple(min(255, c + 40) for c in color)
            elif facet == 2:
                color = tuple(max(0, c - 25) for c in color)
            
            # Destellos verdes
            if random.random() < 0.03:
                color = (150, 255, 180)
            
            pixels[x, y] = color
    
    img.save("assets/textures/emerald.png")
    print("emerald.png")

def generate_obsidian():
    """Genera textura de obsidiana"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (10, 5, 25)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 15)
            
            # Vetas moradas ocasionales
            if random.random() < 0.08:
                color = (
                    color[0] + 15,
                    color[1],
                    min(255, color[2] + 40)
                )
            
            # Reflejos sutiles
            if random.random() < 0.05:
                color = tuple(min(255, c + 50) for c in color)
            
            pixels[x, y] = color
    
    img.save("assets/textures/obsidian.png")
    print("obsidian.png")

def generate_ice():
    """Genera textura de hielo"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (200, 230, 255)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 15)
            
            # Grietas de hielo
            if (x + y) % 7 == 0 or (x - y) % 9 == 0:
                color = tuple(max(0, c - 40) for c in color)
            
            # Brillo cristalino
            if ((x * y) % 5) == 0:
                color = tuple(min(255, c + 20) for c in color)
            
            pixels[x, y] = color
    
    img.save("assets/textures/ice.png")
    print("ice.png")

def generate_brick():
    """Genera textura de ladrillos normales"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    brick_color = (150, 80, 60)
    mortar_color = (180, 180, 180)
    
    for y in range(size):
        for x in range(size):
            offset = (y // 4) % 2 * 4
            if (x + offset) % 8 == 0 or y % 4 == 0:
                pixels[x, y] = add_noise(mortar_color, 10)
            else:
                pixels[x, y] = add_noise(brick_color, 15)
    
    img.save("assets/textures/brick.png")
    print("brick.png (bonus)")

def generate_sand():
    """Genera textura de arena"""
    size = 16
    img = Image.new('RGB', (size, size))
    pixels = img.load()
    
    base_color = (210, 180, 140)
    
    for y in range(size):
        for x in range(size):
            color = add_noise(base_color, 18)
            
            # Granos de arena más oscuros
            if random.random() < 0.03:
                color = tuple(max(0, c - 40) for c in color)
            
            pixels[x, y] = color
    
    img.save("assets/textures/sand.png")
    print("sand.png (bonus)")

def main():
    """Genera todas las texturas"""
    print("=" * 60)
    print("GENERADOR DE TEXTURAS PARA RAY TRACER")
    print("=" * 60)
    
    ensure_directory()
    print("\nGenerando texturas 16x16...\n")
    
    # Texturas básicas del Overworld
    print("Texturas del Overworld:")
    generate_grass_top()
    generate_grass_side()
    generate_dirt()
    generate_stone()
    generate_wood()
    generate_leaves()
    
    # Texturas animadas
    print("\nTexturas animadas:")
    generate_water_animated()
    generate_lava_animated()
    generate_portal_animated()
    
    # Texturas del Nether
    print("\nTexturas del Nether:")
    generate_netherrack()
    generate_nether_brick()
    generate_soul_sand()
    generate_glowstone()
    
    # Texturas de gemas/materiales especiales
    print("\nMateriales especiales:")
    generate_diamond()
    generate_emerald()
    generate_obsidian()
    generate_ice()
    
    # Texturas bonus
    print("\nTexturas bonus:")
    generate_brick()
    generate_sand()
    
    print("\n" + "=" * 60)
    print("¡Todas las texturas generadas exitosamente!")
    print(f"Ubicación: assets/textures/")
    print(f"Total de archivos: 27 texturas")
    print("\nTips:")
    print("   • Edita los PNG con cualquier editor de imágenes")
    print("   • Las texturas animadas tienen sufijos _0, _1, etc.")
    print("   • El programa Rust las cargará automáticamente")
    print("=" * 60)

if __name__ == "__main__":
    try:
        main()
    except ImportError:
        print("=" * 60)
        print("ERROR: PIL (Pillow) no está instalado")
        print("\nInstálalo con:")
        print("   pip install pillow")
        print("\n   o si usas pip3:")
        print("   pip3 install pillow")
        print("=" * 60)
    except Exception as e:
        print(f"Error inesperado: {e}")
        import traceback
        traceback.print_exc()