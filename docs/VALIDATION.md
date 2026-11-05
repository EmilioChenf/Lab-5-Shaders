# Validación real

Ejecutada en Linux x86_64, Rust 1.99.1, Macroquad 0.4.16 (Cargo.lock),
Xorg dummy 1440×900 y Mesa por software. No se ha ejecutado en Windows.

| Comprobación | Resultado |
|---|---|
| `cargo fmt --check` | Correcto |
| `cargo check --locked` | Correcto |
| `cargo test --locked` | 7 aprobadas, 0 fallidas, 0 omitidas |
| `cargo clippy --all-targets --locked -- -D warnings` | Correcto, sin advertencias |
| `cargo build --release --locked` | Correcto |
| Ejecución `--capture-all` | Salida 0, ocho PNG reales |
| Compilación GLSL en contexto OpenGL/Mesa | Ambos materiales cargados sin errores |
| Selección 1–7 y F12 | Eventos reales XTest; archivos actualizados con nombres correctos |
| Pausa/reanudación | Píxeles de superficie idénticos en pausa y diferentes al reanudar |
| Arrastrar mouse | Imagen diferente con animación pausada |
| R | Recupera exactamente los píxeles de la cámara inicial con tiempo pausado |
| Rueda | Cambia la imagen con tiempo pausado |
| 0 y Escape | Captura del atlas y salida 0 |

Las pruebas Rust comprueban índices, radios unitarios, orientación exterior de
triángulos no degenerados, rechazo de overflow u16, anillo con centro abierto,
catálogo con siete nombres exactos y sin colisiones, radio orbital de Nyx, seguimiento de su centro y encuadre independiente, y distancia de cámara.
Los triángulos degenerados de los polos de la esfera se excluyen del test de
orientación; el resto debe apuntar hacia fuera.

Para repetir las comprobaciones de Rust: `sh scripts/check.sh`.
En PowerShell, ejecutar los cinco comandos de la tabla individualmente.

La prueba opcional `scripts/validate_input.py` requiere Linux, un servidor X11
activo en `:99`, `libX11`, `libXtst`, Python y Pillow. Ejecutar desde la raíz con
el binario release ya compilado: `python3 scripts/validate_input.py`.
No instala paquetes. Lanza y cierra su propia aplicación e **intercambia las
capturas existentes** al probar F12. Después se pueden regenerar las imágenes
editoriales mediante `cargo run --release --locked -- --capture-all`.

La inspección visual confirmó cuerpos diferenciados, anillos con huecos y
oclusión por el planeta, y luna separada. El shader común recibe `Kind` distinto
para los seis cuerpos principales y Nyx; los anillos usan otro fragment shader. No se
cargan imágenes para las superficies: los PNG son exclusivamente evidencia.
El redimensionamiento conserva el aspect ratio mediante Camera3D y las
coordenadas de UI dependen de las dimensiones actuales; la interacción manual
en Windows y la cobertura de otras GPU quedan pendientes.

## Captura automática actualizada

`python3 scripts/capture_all.py` compila release, inicia `--capture-all` y
verifica ocho PNG nuevos, de 1440×900 en esta sesión, con superficie visible y
huellas de contenido distintas. La octava vista es Nyx, la luna independiente;
Verdantis conserva su nombre y aspecto. Mambo se encuadra a distancia 8 para
mostrar los anillos completos. Los siete shaders conservan sus fórmulas.

Se repitieron las pruebas de entrada con las siete teclas y los nombres finales.
Las imágenes se regeneraron después para eliminar las capturas de prueba.
