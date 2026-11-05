"""Audita cuatro contribuciones mediante omisiones en renders reales, nunca imágenes sintéticas."""
from pathlib import Path
import json
import os
import platform
import subprocess
from PIL import Image, ImageChops
import capture_all

ROOT = capture_all.ROOT
OUTPUT = ROOT / "docs" / "audit" / "layers"


def viewport(path):
    with Image.open(path) as image:
        image.load()
        if image.format != "PNG" or image.size != (1440, 900):
            raise RuntimeError(f"FAIL: formato o tamaño inesperado: {path}")
        # Same simulation, mesh, camera and lighting. Exclude text and variable FPS.
        return image.convert("RGB").crop((0, 90, 1440, 856))


def main():
    capture_all.main()
    names = capture_all.NAMES[1:]
    files = [OUTPUT / name / f"variante_{i}.png" for name in names for i in range(5)]
    previous = {path: path.stat().st_mtime_ns if path.exists() else None for path in files}
    binary = ROOT / "target" / "release" / ("lab-5-shaders.exe" if os.name == "nt" else "lab-5-shaders")
    negative = subprocess.run([str(binary), "--check-shader-errors"], cwd=ROOT, check=True, timeout=30, capture_output=True, text=True)
    (OUTPUT.parent / "shader_errors.txt").write_text(negative.stdout + negative.stderr, encoding="utf-8")
    subprocess.run([str(binary), "--audit-layers"], cwd=ROOT, check=True, timeout=120)
    for path in files:
        if not path.is_file() or path.stat().st_mtime_ns == previous[path]:
            raise RuntimeError(f"FAIL: evidencia no renovada: {path}")
    results = []
    for name in names:
        base = viewport(OUTPUT / name / "variante_0.png")
        production = viewport(capture_all.OUTPUT / f"{name}.png")
        if ImageChops.difference(base, production).getbbox() is not None:
            raise RuntimeError(f"FAIL: producción y las cuatro capas habilitadas difieren: {name}")
        for layer in range(1, 5):
            variant = viewport(OUTPUT / name / f"variante_{layer}.png")
            pixels = ImageChops.difference(base, variant).tobytes()
            changed = sum(max(pixels[i:i+3]) > 1 for i in range(0, len(pixels), 3))
            if changed < 100:
                raise RuntimeError(f"FAIL: contribución sin evidencia suficiente: {name}/{layer}: {changed}")
            result = {"body": name, "omitted_layer": layer, "changed_pixels_rgb_gt_1": changed, "status": "PASS"}
            results.append(result)
            print(f"PASS: {name}, capa {layer}: {changed} píxeles modificados")
    report = {
        "status": "PASS", "method": "four leave-one-out renders against all-layers baseline; fixed time=3; HUD excluded",
        "shader_source_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "platform": platform.platform(),
        "mesa_gl_override": os.environ.get("MESA_GL_VERSION_OVERRIDE"),
        "mesa_glsl_override": os.environ.get("MESA_GLSL_VERSION_OVERRIDE"),
        "production_matches_all_layers": True, "fresh_png_count": len(files), "results": results,
    }
    (OUTPUT.parent / "layers.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print("PASS: 28 contribuciones observadas, 35 renders nuevos, 7 vistas iguales a producción")


if __name__ == "__main__":
    main()
