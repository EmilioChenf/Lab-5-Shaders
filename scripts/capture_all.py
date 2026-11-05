"""Compila, captura y verifica las ocho vistas reales. Requiere Pillow y sesión gráfica."""
from pathlib import Path
import hashlib
import os
import subprocess
from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
NAMES = ["atlas", "PAPATIN", "PAPATINSENIOR", "MAMBO", "CRYON", "CITOPA", "VERDANTIS", "NYX"]
OUTPUT = ROOT / "docs" / "screenshots"


def main():
    subprocess.run(["cargo", "build", "--release", "--locked"], cwd=ROOT, check=True)
    previous = {
        name: (OUTPUT / f"{name}.png").stat().st_mtime_ns
        if (OUTPUT / f"{name}.png").exists() else None
        for name in NAMES
    }
    binary = ROOT / "target" / "release" / ("lab-5-shaders.exe" if os.name == "nt" else "lab-5-shaders")
    subprocess.run([str(binary), "--capture-all"], cwd=ROOT, check=True, timeout=120)
    fingerprints = set()
    for name in NAMES:
        path = OUTPUT / f"{name}.png"
        if not path.is_file() or path.stat().st_mtime_ns == previous[name]:
            raise RuntimeError(f"Captura no actualizada: {path}")
        with Image.open(path) as image:
            image.load()
            if image.format != "PNG" or image.width < 1280 or image.height < 720:
                raise RuntimeError(f"PNG o resolución incorrecta: {path}")
            # Exclude HUD; a real body must occupy part of the central viewport.
            center = image.convert("RGB").crop((image.width//4, image.height//4, image.width*3//4, image.height*3//4))
            colors = center.getdata()
            bright = sum(max(pixel) > 70 for pixel in colors)
            if bright < center.width * center.height // 100:
                raise RuntimeError(f"Superficie ausente o imagen vacía: {path}")
            digest = hashlib.sha256(center.tobytes()).hexdigest()
            if digest in fingerprints:
                raise RuntimeError(f"Vista duplicada: {path}")
            fingerprints.add(digest)
            print(f"VERIFICADA: {path.name} ({image.width}x{image.height})")
    print("Ocho capturas nuevas, válidas y diferentes.")


if __name__ == "__main__":
    main()
