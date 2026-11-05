# Revisión y propuesta de historial — pendiente de autorización

Se revisaron los **19 commits anteriores** hasta `2acc888`, ambos extremos
incluidos. Todos tienen nombre de autor `EmilioChenf`. El inicial tiene el correo
noreply de GitHub; los otros 18 usan el correo anterior configurado en el
repositorio. Los 19 mensajes están en inglés.

Los commits nuevos ya usan `EmilioChenf <che24841@uvg.edu.gt>` y mensajes en
español. No se modificó la configuración global. GitHub requiere que el correo
académico esté añadido y verificado en la cuenta para asociarlo automáticamente;
la configuración de Git no demuestra esa verificación.

## Propuesta concreta

- En los 18 commits posteriores al inicial, cambiar autor y committer a la
  identidad académica solicitada, conservando el contenido y el orden.
- Conservar la identidad noreply del commit inicial: ya figura como EmilioChenf.
  No se atribuirán aportes de terceros. Si se identifica una contribución ajena,
  preservar su autoría.
- Traducir los 19 mensajes según la tabla siguiente; conservar las fechas de
  autor registradas. Las fechas de committer pueden conservarse explícitamente
  al reconstruir los commits, aunque los hashes cambiarán por los metadatos y
  la cadena de padres. Las firmas existentes no sobrevivirían una reescritura.
- El commit inicial cambia de hash si se traduce su mensaje; por tanto cambiarán
  también todos sus descendientes, incluidos los nuevos commits de esta revisión.

| Hash actual | Fecha original de autor | Mensaje propuesto |
|---|---|---|
| `d6405fc` | 2026-11-04T19:02:57-06:00 | inicio: crear repositorio del laboratorio |
| `d4a1283` | 2026-11-05T01:28:06Z | preparación: inicializar laboratorio de shaders en Rust |
| `b7ae230` | 2026-11-05T01:28:26Z | funcionalidad: generar esfera y anillo procedurales |
| `fe7d8a6` | 2026-11-05T01:28:47Z | funcionalidad: incorporar GLSL embebido y ruido local |
| `5ec4484` | 2026-11-05T01:29:06Z | funcionalidad: animar plasma y manchas estelares |
| `3420de9` | 2026-11-05T01:29:06Z | funcionalidad: implementar basalto y fisuras de lava |
| `d12557f` | 2026-11-05T01:29:07Z | funcionalidad: crear bandas atmosféricas y gran tormenta |
| `020dd50` | 2026-11-05T01:29:28Z | funcionalidad: implementar glaciares nieve y fracturas |
| `dded178` | 2026-11-05T01:29:28Z | funcionalidad: crear nubes energéticas y venas luminosas |
| `e3cd7a2` | 2026-11-05T01:29:28Z | funcionalidad: implementar biomas y superficie lunar con cráteres |
| `a175ad3` | 2026-11-05T01:29:51Z | funcionalidad: añadir anillos independientes y órbita lunar |
| `58998d3` | 2026-11-05T01:29:51Z | funcionalidad: implementar cámara orbital y zoom limitado |
| `732bb4e` | 2026-11-05T01:31:00Z | funcionalidad: renderizar atlas con selección animación y capturas |
| `c851ed1` | 2026-11-05T01:32:37Z | corrección: evitar truncamiento y validar geometría órbita y cámara |
| `9425c12` | 2026-11-05T01:33:17Z | corrección: usar proyección en radianes y aislar sistemas seleccionados |
| `f70114d` | 2026-11-05T01:33:39Z | corrección: mantener anillos al seleccionar el gigante gaseoso |
| `7f01b1e` | 2026-11-05T01:34:56Z | pruebas: añadir validación reproducible de Rust y controles gráficos |
| `51db89b` | 2026-11-05T01:35:05Z | documentación: añadir siete capturas reales del renderizador |
| `2acc888` | 2026-11-05T01:36:18Z | documentación: explicar shaders instalación controles y resultados |

## Procedimiento que se aplicaría únicamente con autorización específica

1. Hacer `fetch`, comprobar limpieza y que el HEAD remoto coincida con el SHA
   esperado. Registrar ese SHA; detenerse si aparece trabajo remoto nuevo.
2. Crear una copia recuperable completa con `git bundle create` y comprobarla
   con `git bundle verify`; guardar el respaldo fuera del checkout.
3. Reconstruir los commits en una referencia temporal local, sin cambiar `main`,
   aplicando solo los metadatos y mensajes acordados. Conservar árbol, fechas
   originales cuando sea posible, relaciones y cualquier trabajo posterior.
4. Verificar igualdad de árboles finales y de cada commit correspondiente,
   fechas, identidad, número y orden. Repetir las comprobaciones del proyecto.
5. Presentar la correspondencia completa de hashes para revisión. Solo tras la
   autorización específica de publicación, actualizar el remoto con una
   protección `--force-with-lease` ligada al SHA remoto previamente registrado.
   Nunca usar `--force` ni publicar si cambió la referencia remota.
6. Coordinar con cualquier colaborador que tenga clones: deberán reconciliar
   su historia con la nueva. Los enlaces a commits, ramas derivadas y revisiones
   pueden quedar referidos al historial anterior; conservar el respaldo.

**Estado real:** solo se preparó esta propuesta. No se reconstruyeron commits,
no se movieron referencias históricas y no se hizo force push. Los cambios del
laboratorio se publican mediante push normal, separado de esta posible operación.

Una alternativa sin reescritura es verificar en GitHub un correo anterior que
realmente pertenezca al usuario; esto puede corregir la asociación de cuenta,
pero no cambia los mensajes ni el correo almacenado en los commits.
