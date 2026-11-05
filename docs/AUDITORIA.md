# Auditoría final del Laboratorio 5

**Estudiante:** Emilio Chen · **GitHub:** EmilioChenf · **Correo:** che24841@uvg.edu.gt

## Alcance y nombres confirmados

Se auditó código Rust, GLSL, geometría, flujo de uniformes, pruebas, renderizado
real y documentación. Se ejecutaron los comandos, no se tomó el README como
prueba de funcionamiento. Estados utilizados: **PASS**, **FAIL**, **NO VERIFICADO**.

El adjunto de auditoría invertía los nombres del planeta verde y la luna.
El usuario confirmó expresamente **VERDANTIS planeta y NYX luna**. Esta aclaración
prevalece; no se contó una luna como planeta adicional.

| Índice / tecla | Nombre | Papel | Shader / rama `Kind` | Estado |
|---|---|---|---|---|
| 0 / 1 | PAPATIN | Estrella obligatoria, emisiva | `body.frag`, `< .5` | PASS |
| 1 / 2 | PAPATINSENIOR | Rocoso volcánico obligatorio | `body.frag`, `< 1.5` | PASS |
| 2 / 3 | MAMBO | Gigante gaseoso obligatorio | `body.frag`, `< 2.5` | PASS |
| 3 / 4 | CRYON | Planeta adicional 1, helado | `body.frag`, `< 3.5` | PASS |
| 4 / 5 | CITOPA | Planeta adicional 2, energía oscura | `body.frag`, `< 4.5` | PASS |
| 5 / 6 | VERDANTIS | Planeta adicional 3, biomas y océanos | `body.frag`, `< 5.5` | PASS |
| 6 / 7 | NYX | Luna de PAPATINSENIOR; excluida del cómputo de planetas | `body.frag`, `else` | PASS |

Las siete superficies comparten un programa GLSL y un material personalizado,
pero cada rama tiene su propia composición procedural, verificada por renders.
No se afirma que existan siete programas GPU separados. El anillo utiliza otro
fragment shader y otra malla. Compartir la malla base de esfera no fusiona los
objetos: se dibujan con centros, radios y rotaciones independientes.

## Evaluación de la rúbrica

| Criterio | Máximo | Estado | Puntos estimados | Evidencia | Correcciones realizadas |
|---|---:|---|---:|---|---|
| Creatividad | 30 | PASS | 26 | Ocho vistas reales; seis diseños distinguibles, incandescencia, lava, bandas/tormenta, hielo, energía y biomas | Etiquetas del atlas fuera de anillos y tránsito lunar; encuadre de MAMBO |
| Complejidad de shaders | 40 | PASS | 40 | 24 contribuciones de cuerpos principales y 4 de la luna, con omisiones verificadas en GPU | Instrumentación `LayerMask`, GLSL de escritorio y prueba de rechazo de código inválido |
| Planetas extras | 30 | PASS | 30 | CRYON, CITOPA y VERDANTIS, posiciones y selecciones independientes | Aclaración de nombres; NYX sigue siendo luna |
| Anillos | 20 | PASS | 20 | Geometría anular sin texturas, vistas superior/inferior y prueba de 20 orientaciones | Distancia inicial de cámara de MAMBO ampliada de 8 a 10 |
| Luna | 20 | PASS | 20 | NYX independiente, cráteres procedurales, órbita comprobada y tecla 7 | Etiqueta asociada a su padre en atlas, sin texto sobre el tránsito |

**Estimación técnica: 136/140 puntos disponibles. Máximo oficial aplicable: 100
puntos.** No se convierte esta estimación automáticamente en una nota 100/100;
la interpretación y calificación corresponden al profesor.

La creatividad se evalúa razonadamente, no por contar funciones: los colores y
frecuencias distinguen los seis diseños y cada uno presenta detalle visible.
No se asigna 30/30 porque la iluminación es artística/direccional, las fisuras y
venas tienen frecuencia visual fuerte y no hay sombras proyectadas entre cuerpos
ni bloom exterior. Nada de esto se presenta como una simulación física completa.
La valoración artística y la calificación oficial son **NO VERIFICADO** por esta
auditoría; el PASS de creatividad indica diseños y diferencias observados.

## Cuatro contribuciones reales por superficie

Archivo común: [`shaders/body.frag`](../shaders/body.frag), función `main`,
con una rama por cuerpo. `n=fbm(p*5)` y `detail=fbm(p*28)` usan hash y value noise
trilineal de cinco octavas. `p` es la posición local normalizada, no coordenadas
de pantalla. `rim` depende de la normal rotada y de la dirección a `Eye`.

En la tabla, **todos** los renglones utilizan `Kind` para seleccionar la rama y
`LayerMask.x/y/z/w` para habilitar la capa 1/2/3/4. La columna de uniformes añade
los utilizados directamente por esa contribución; `—` significa ruido o máscaras
estáticas en espacio local, no que la contribución carezca de efecto. `Spin`,
`Center`, `Radius`, `Tilt`, `Model` y `Projection` pertenecen al vertex shader y
posicionan/rotan las siete superficies; no se cuentan como capas adicionales.

| Cuerpo / capa | Nombre | Archivo y bloque | Uniformes adicionales | Operación que modifica el color final | Píxeles cambiados al omitirla | Estado |
|---|---|---|---|---|---:|---|
| PAPATIN / 1 | Plasma convectivo | [`body.frag:L28`](../shaders/body.frag#L28), `main` rama PAPATIN | Time; `Kind`, `LayerMask.x` | Asignación `mix(rojo, amarillo, plasma)` | 255235 | PASS |
| PAPATIN / 2 | Manchas solares | [`body.frag:L29`](../shaders/body.frag#L29), `main` rama PAPATIN | Time; `Kind`, `LayerMask.y` | Mezcla con marrón oscuro mediante `spots` | 30594 | PASS |
| PAPATIN / 3 | Filamentos calientes | [`body.frag:L30`](../shaders/body.frag#L30), `main` rama PAPATIN | Time (a través de plasma); `Kind`, `LayerMask.z` | Suma naranja modulada por `filaments` | 74471 | PASS |
| PAPATIN / 4 | Borde pulsante | [`body.frag:L31`](../shaders/body.frag#L31), `main` rama PAPATIN | Time, Eye; `Kind`, `LayerMask.w` | Suma de `rim` con intensidad sinusoidal | 156027 | PASS |
| PAPATINSENIOR / 1 | Basalto | [`body.frag:L38`](../shaders/body.frag#L38), `main` rama PAPATINSENIOR | —; `Kind`, `LayerMask.x` | Asignación mineral oscura según `n` | 68239 | PASS |
| PAPATINSENIOR / 2 | Ceniza granular | [`body.frag:L39`](../shaders/body.frag#L39), `main` rama PAPATINSENIOR | —; `Kind`, `LayerMask.y` | Suma gris/cálida según `detail` | 64418 | PASS |
| PAPATINSENIOR / 3 | Fisuras de lava | [`body.frag:L40`](../shaders/body.frag#L40), `main` rama PAPATINSENIOR | —; `Kind`, `LayerMask.z` | Mezcla roja mediante máscara `cracks` | 34545 | PASS |
| PAPATINSENIOR / 4 | Bolsas incandescentes | [`body.frag:L41`](../shaders/body.frag#L41), `main` rama PAPATINSENIOR | Time; `Kind`, `LayerMask.w` | Suma naranja mediante máscara `hot` | 13012 | PASS |
| MAMBO / 1 | Bandas crema/marrón | [`body.frag:L47`](../shaders/body.frag#L47), `main` rama MAMBO | Time; `Kind`, `LayerMask.x` | Asignación por bandas sinusoidales distorsionadas | 68511 | PASS |
| MAMBO / 2 | Cinturones ocres | [`body.frag:L48`](../shaders/body.frag#L48), `main` rama MAMBO | —; `Kind`, `LayerMask.y` | Mezcla ocre por umbral de `n` | 24079 | PASS |
| MAMBO / 3 | Tormenta oval | [`body.frag:L51`](../shaders/body.frag#L51), `main` rama MAMBO | —; `Kind`, `LayerMask.z` | Mezcla rojiza con anillos concéntricos y máscara `storm` | 809 | PASS |
| MAMBO / 4 | Nubes finas | [`body.frag:L52`](../shaders/body.frag#L52), `main` rama MAMBO | —; `Kind`, `LayerMask.w` | Suma de variación clara según `detail` | 68808 | PASS |
| CRYON / 1 | Glaciares azules | [`body.frag:L56`](../shaders/body.frag#L56), `main` rama CRYON | —; `Kind`, `LayerMask.x` | Asignación azul con variación de `n` | 78834 | PASS |
| CRYON / 2 | Nieve y casquetes | [`body.frag:L58`](../shaders/body.frag#L58), `main` rama CRYON | —; `Kind`, `LayerMask.y` | Mezcla blanca según `snow` y latitud | 82924 | PASS |
| CRYON / 3 | Fracturas | [`body.frag:L60`](../shaders/body.frag#L60), `main` rama CRYON | —; `Kind`, `LayerMask.z` | Mezcla azul oscura por `crack` | 24435 | PASS |
| CRYON / 4 | Brillos y limbo de hielo | [`body.frag:L61`](../shaders/body.frag#L61), `main` rama CRYON | Eye; `Kind`, `LayerMask.w` | Suma azul/blanca por detalle de alta frecuencia y `rim` | 91673 | PASS |
| CITOPA / 1 | Obsidiana violeta | [`body.frag:L67`](../shaders/body.frag#L67), `main` rama CITOPA | —; `Kind`, `LayerMask.x` | Asignación de terreno oscuro según `n` | 119743 | PASS |
| CITOPA / 2 | Nubes energéticas | [`body.frag:L68`](../shaders/body.frag#L68), `main` rama CITOPA | Time; `Kind`, `LayerMask.y` | Suma violeta por umbral de `cloud` | 66265 | PASS |
| CITOPA / 3 | Venas eléctricas | [`body.frag:L69`](../shaders/body.frag#L69), `main` rama CITOPA | Time; `Kind`, `LayerMask.z` | Suma cian de crestas estrechas `vein` | 16993 | PASS |
| CITOPA / 4 | Halo del limbo | [`body.frag:L70`](../shaders/body.frag#L70), `main` rama CITOPA | Eye; `Kind`, `LayerMask.w` | Suma azul según `rim` | 73880 | PASS |
| VERDANTIS / 1 | Océanos turquesa | [`body.frag:L76`](../shaders/body.frag#L76), `main` rama VERDANTIS | —; `Kind`, `LayerMask.x` | Asignación marina según `detail` | 74267 | PASS |
| VERDANTIS / 2 | Continentes verdes | [`body.frag:L77`](../shaders/body.frag#L77), `main` rama VERDANTIS | —; `Kind`, `LayerMask.y` | Mezcla vegetal por máscara `land` | 73911 | PASS |
| VERDANTIS / 3 | Cordilleras minerales | [`body.frag:L78`](../shaders/body.frag#L78), `main` rama VERDANTIS | —; `Kind`, `LayerMask.z` | Mezcla parda en alturas procedurales de `n` | 10654 | PASS |
| VERDANTIS / 4 | Nubes móviles | [`body.frag:L80`](../shaders/body.frag#L80), `main` rama VERDANTIS | Time; `Kind`, `LayerMask.w` | Mezcla blanca según `clouds` | 23247 | PASS |
| NYX / 1 | Regolito moteado | [`body.frag:L88`](../shaders/body.frag#L88), `main` rama NYX | —; `Kind`, `LayerMask.x` | Asignación gris según `n` | 50555 | PASS |
| NYX / 2 | Tierras claras | [`body.frag:L89`](../shaders/body.frag#L89), `main` rama NYX | —; `Kind`, `LayerMask.y` | Suma de detalle fino de superficie | 50534 | PASS |
| NYX / 3 | Depresiones de cráter | [`body.frag:L90`](../shaders/body.frag#L90), `main` rama NYX | —; `Kind`, `LayerMask.z` | Multiplicación oscurecedora por celdas habilitadas | 1773 | PASS |
| NYX / 4 | Bordes de cráter | [`body.frag:L91`](../shaders/body.frag#L91), `main` rama NYX | —; `Kind`, `LayerMask.w` | Suma gaussiana clara alrededor de cada depresión | 2435 | PASS |

Las operaciones anteriores desembocan en la expresión real
`gl_FragColor = vec4(color * mix(light, 1., emission), 1.)`. No se cuenta una
variable sin uso ni se cuenta la iluminación general como una cuarta capa.
VERDANTIS tiene además un borde atmosférico que no es necesario para completar
las cuatro contribuciones de la tabla. Las montañas y cráteres son apariencia
de color, no desplazamiento de la geometría de esfera.

Las capas son independientes en su contribución compositiva: retirar cualquiera
cambia la imagen final. Comparten ruido y pueden solaparse; no se afirma
independencia estadística ni que todos los patrones sean visibles desde todos
los ángulos (por ejemplo, la tormenta está en un hemisferio de MAMBO).

### Método experimental

`python scripts/audit_layers.py` ejecuta el binario real con cámara y tiempo
fijos (3 s), cinco variantes por cuerpo: 0 = todo activo; 1–4 = omitir esa capa.
Se esperan ocho fotogramas por variante antes de capturar. Las comparaciones
excluyen la interfaz/FPS y cuentan píxeles cuya diferencia RGB supera 1 nivel
sobre 255. Cada contribución debe modificar al menos 100 píxeles. La luna queda
sin cambios al auditar la superficie de su padre, para no atribuirle sus píxeles.

PASS: 35 PNG nuevos y 28 contribuciones comprobadas. Las siete variantes 0 son
idénticas a las superficies de producción. Los números y el commit de código
que produjo los renders están en [layers.json](audit/layers.json); la salida
completa está en [layers_run.txt](audit/layers_run.txt). Los controles de capa
solo se alteran en auditoría; producción utiliza `vec4(1)`.

| Cuerpo | Render completo | Omitir capa 1 | Omitir capa 2 | Omitir capa 3 | Omitir capa 4 |
|---|---|---|---|---|---|
| PAPATIN | [PNG 0](audit/layers/PAPATIN/variante_0.png) | [PNG 1](audit/layers/PAPATIN/variante_1.png) | [PNG 2](audit/layers/PAPATIN/variante_2.png) | [PNG 3](audit/layers/PAPATIN/variante_3.png) | [PNG 4](audit/layers/PAPATIN/variante_4.png) |
| PAPATINSENIOR | [PNG 0](audit/layers/PAPATINSENIOR/variante_0.png) | [PNG 1](audit/layers/PAPATINSENIOR/variante_1.png) | [PNG 2](audit/layers/PAPATINSENIOR/variante_2.png) | [PNG 3](audit/layers/PAPATINSENIOR/variante_3.png) | [PNG 4](audit/layers/PAPATINSENIOR/variante_4.png) |
| MAMBO | [PNG 0](audit/layers/MAMBO/variante_0.png) | [PNG 1](audit/layers/MAMBO/variante_1.png) | [PNG 2](audit/layers/MAMBO/variante_2.png) | [PNG 3](audit/layers/MAMBO/variante_3.png) | [PNG 4](audit/layers/MAMBO/variante_4.png) |
| CRYON | [PNG 0](audit/layers/CRYON/variante_0.png) | [PNG 1](audit/layers/CRYON/variante_1.png) | [PNG 2](audit/layers/CRYON/variante_2.png) | [PNG 3](audit/layers/CRYON/variante_3.png) | [PNG 4](audit/layers/CRYON/variante_4.png) |
| CITOPA | [PNG 0](audit/layers/CITOPA/variante_0.png) | [PNG 1](audit/layers/CITOPA/variante_1.png) | [PNG 2](audit/layers/CITOPA/variante_2.png) | [PNG 3](audit/layers/CITOPA/variante_3.png) | [PNG 4](audit/layers/CITOPA/variante_4.png) |
| VERDANTIS | [PNG 0](audit/layers/VERDANTIS/variante_0.png) | [PNG 1](audit/layers/VERDANTIS/variante_1.png) | [PNG 2](audit/layers/VERDANTIS/variante_2.png) | [PNG 3](audit/layers/VERDANTIS/variante_3.png) | [PNG 4](audit/layers/VERDANTIS/variante_4.png) |
| NYX | [PNG 0](audit/layers/NYX/variante_0.png) | [PNG 1](audit/layers/NYX/variante_1.png) | [PNG 2](audit/layers/NYX/variante_2.png) | [PNG 3](audit/layers/NYX/variante_3.png) | [PNG 4](audit/layers/NYX/variante_4.png) |

## Compatibilidad y transferencia de uniformes

- PASS: shaders nativos `#version 120`, cargados en un contexto Mesa con
  `MESA_GL_VERSION_OVERRIDE=2.1` y `MESA_GLSL_VERSION_OVERRIDE=120`. Se corrigió
  la declaración ES 1.00 anterior para corresponder al backend de escritorio.
- PASS: `src/shaders.rs` declara tipos que coinciden con GLSL: `Time`, `Kind`,
  `Spin`, `Radius`, `Tilt` como float; `Eye`, `Center` como vec3 y `LayerMask`
  como vec4. Las 28 comparaciones comprueban su uso efectivo en GPU.
- PASS: `include_str!` embebe las fuentes; no dependen del directorio de trabajo.
  `Renderer::new` propaga errores y la aplicación falla explícitamente si no
  puede compilar, sin reemplazar cuerpos por un color fijo.
- PASS: el modo `--check-shader-errors` usa el mismo cargador con GLSL inválido,
  recibe un error y termina tras comprobarlo. [Salida](audit/shader_errors.txt).
- PASS: coordenadas locales sin rotar alimentan el ruido; posiciones y normales
  giran juntas en `body.vert`. Solo hay escala uniforme positiva. El efecto
  permanece unido al objeto al inspeccionarlo y animarlo.
- PASS: no hay `sampler` ni consultas de textura en los shaders de superficie o
  anillos; `Mesh.texture` es `None`. No hay carga de PNG/JPG para la superficie.
  La fuente de interfaz de Macroquad puede usar un atlas interno: no interviene
  en los colores de los cuerpos y no se confunde con una textura superficial.
- PASS: iluminación ambiente/direccional sobre normales normalizadas; estrella
  emisiva y emisión parcial de lava/energía. La luz no se calcula físicamente
  desde la posición de PAPATIN, y el borde luminoso no es bloom exterior.

## Renderizado, anillos y luna

PASS: la aplicación abrió una ventana real en Xorg dummy y dibujó las ocho
vistas. La inspección visual abrió los ocho PNG: no hubo pantalla negra, cuerpos
planos ni sustitución silenciosa de shaders. Los patrones distinguen claramente
plasma naranja, basalto/lava, bandas doradas con tormenta, hielo blanco/azul,
energía violeta y biomas verdes/turquesa. La luna presenta moteado y cráteres.

PASS: anillos separados, radios 1.35–2.15 veces el radio del planeta, inclinación
0.38 radianes, bandas radiales y huecos por `discard`. Depth test `LessOrEqual`,
escritura de profundidad y ausencia de culling permiten oclusión con MAMBO y
visibilidad de ambas caras. No usan alpha/transparencia para simular la malla.
Se observaron ambos lados mediante eventos reales de mouse y se verificaron
píxeles dorados fuera de la esfera. Una prueba adicional proyecta los vértices
en 20 orientaciones para mantenerlos fuera de las bandas de interfaz al usar
la distancia inicial corregida. El zoom deliberado puede ampliar y recortar
objetos; no se promete encuadre completo a cualquier zoom.

Los anillos son superficies sin espesor: exactamente de canto su área proyectada
tiende a cero, comportamiento geométrico esperado. No se afirma volumen ni una
verificación de infinitas orientaciones; se comprobaron ambos lados y los casos
muestreados. Las estrellas del fondo son puntos de geometría, no imágenes.

| Vista de cámara real | Evidencia | Estado |
|---|---|---|
| Frontal | [PNG](audit/views/MAMBO_frontal.png) | PASS |
| Superior | [PNG](audit/views/MAMBO_superior.png) | PASS |
| Inferior | [PNG](audit/views/MAMBO_inferior.png) | PASS |
| Luna a 1280×720 tras redimensionar | [PNG](audit/views/NYX_1280x720.png) | PASS |

PASS: NYX se dibuja en su propia posición, orbita PAPATINSENIOR y nunca toca su
superficie: la distancia entre centros es `sqrt(1.75²+0.25²)`, mayor que la suma
de radios `1+0.29`. Se prueba a lo largo de 200 tiempos, además de visualizarla
con tecla 7. Que pase delante de su padre en una captura es un tránsito, no una
intersección geométrica. No se incluye entre los tres planetas adicionales.

## Pruebas ejecutadas y resultados reales

| Comando o prueba | Estado | Resultado / evidencia |
|---|---|---|
| `cargo fmt --check` | PASS | Salida 0; [registro](audit/rust.txt) |
| `cargo check --locked` | PASS | Salida 0 |
| `cargo test --locked` | PASS | 8 aprobadas, 0 fallidas, 0 omitidas |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS | Salida 0, sin advertencias |
| `cargo build --release --locked` | PASS | Salida 0 |
| `cargo check --locked --offline --target x86_64-pc-windows-gnu` | PASS | Comprobación para Windows GNU; [registro](audit/windows_check.txt) |
| Ejecución nativa Windows/MSVC y GPU Windows | NO VERIFICADO | No hay sesión Windows, toolchain de enlace MSVC ni Wine; no se afirma haber generado o ejecutado un EXE |
| `python3 scripts/validate_input.py` | PASS | Selección 1–7, F12, pausa, reanudación, mouse, R, zoom, 0 y Escape; [registro](audit/input.txt) |
| Renders de anillos superior/inferior | PASS | Evidencias reales de ambas caras |
| Redimensionamiento X11 | PASS | PNG nuevo exactamente 1280×720 |
| `python3 scripts/audit_layers.py` | PASS | Ocho capturas renovadas, 35 renders y 28 contribuciones |
| Otras GPU, macOS/Metal o WebGL | NO VERIFICADO | Fuera de la sesión y del backend de escritorio preparado |

Las pruebas Rust cubren índices, orientación exterior, esfera unitaria, límite
u16, centro abierto del anillo, nombres exactos, separación de cuerpos,
trayectoria lunar, seguimiento del objetivo, distancia de cámara y encuadre de
anillos. Los triángulos degenerados en los polos no se cuentan como caras con
normal válida. La pausa y R se verificaron por igualdad de píxeles; reanudar,
arrastrar y hacer zoom deben producir diferencias reales.

## Capturas y README

PASS: `atlas.png`, `PAPATIN.png`, `PAPATINSENIOR.png`, `MAMBO.png`, `CRYON.png`,
`CITOPA.png`, `NYX.png` y `VERDANTIS.png` son PNG decodificables de 1440×900,
con contenido visible y huellas diferentes. El verificador exige que se hayan
renovado durante su ejecución. Se regeneraron después de las pruebas de entrada
para no dejar una vista accidental de zoom o tamaño reducido como evidencia
editorial. [Manifiesto con dimensiones y SHA-256](audit/screenshots.json).

PASS: el README incluye estudiante, correo, GitHub, objetivo, tecnologías,
requisitos, Windows, controles, cuerpos, cuatro capas, anillos, luna, pruebas y
las ocho imágenes con rutas existentes. Los enlaces locales se comprobaron.

## Hallazgos y correcciones

| Hallazgo inicial | Estado inicial | Solución | Estado final |
|---|---|---|---|
| Renders de capas sin experimento de contribución individual | NO VERIFICADO | `LayerMask`, 35 capturas y 28 comparaciones medibles | PASS |
| Declaración GLSL ES en backend de escritorio | NO VERIFICADO | GLSL 1.20 y ejecución limitada a OpenGL 2.1 | PASS |
| Parte de los anillos bajo la interfaz al orbitar | FAIL | Encuadre de MAMBO a distancia 10 y prueba de proyección | PASS |
| Etiquetas del atlas sobre anillo y tránsito lunar | FAIL | Proyectar borde del anillo y agrupar etiqueta padre+luna | PASS |
| Comprobación real de redimensionamiento ausente | NO VERIFICADO | Evento X11, captura 1280×720 e inspección visual | PASS |
| Rechazo de GLSL inválido sin prueba negativa | NO VERIFICADO | Mismo cargador rechaza fragment shader malformado | PASS |
| Ejecución real en Windows | NO VERIFICADO | Comprobación de código para Windows GNU; límite documentado | NO VERIFICADO |

No quedan FAIL técnicos abiertos de los criterios funcionales en la plataforma
probada. Quedan NO VERIFICADO la ejecución Windows/otras GPU y la valoración
oficial. No se reemplazan esas validaciones por una afirmación de nota máxima.
Las funcionalidades de la rúbrica están comprobadas en Linux, con la asignación
de nombres expresamente confirmada por el usuario.

## Reproducción y Git

Con Rust, Pillow, X11 y Mesa disponibles, desde la raíz:

```sh
sh scripts/check.sh
DISPLAY=:99 LIBGL_ALWAYS_SOFTWARE=1 MESA_GL_VERSION_OVERRIDE=2.1 MESA_GLSL_VERSION_OVERRIDE=120 python3 scripts/validate_input.py
DISPLAY=:99 LIBGL_ALWAYS_SOFTWARE=1 MESA_GL_VERSION_OVERRIDE=2.1 MESA_GLSL_VERSION_OVERRIDE=120 python3 scripts/audit_layers.py
```

La prueba de entrada crea y termina su propia aplicación; no se debe ejecutar
simultáneamente con otro proceso que capture el foco de esa ventana. El último
comando restaura las ocho vistas editoriales y verifica las contribuciones. En
Windows se usan los comandos Cargo documentados y los scripts de captura dentro
de una sesión gráfica; el test X11 no corresponde a esa plataforma.

Los nuevos commits usan `EmilioChenf <che24841@uvg.edu.gt>` y mensajes en español.
Se mantiene `main`, con push normal a
<https://github.com/EmilioChenf/Lab-5-Shaders>. No se reescribe el historial remoto.
El reporte de chat final registra los hashes nuevos, el resultado real del push
y la comparación de `HEAD`, `origin/main` y la referencia de GitHub después de
publicar; no se inventa aquí un hash de un commit que todavía no existe.
