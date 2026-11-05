use crate::{bodies, camera::Orbit, render::Renderer};
use macroquad::prelude::*;
pub fn configuration() -> macroquad::conf::Conf {
    let window = Conf {
        window_title: "Laboratorio 5 | Atlas de mundos procedurales".into(),
        window_width: 1440,
        window_height: 900,
        window_resizable: true,
        ..Default::default()
    };
    macroquad::conf::Conf {
        miniquad_conf: window,
        draw_call_vertex_capacity: 10000,
        draw_call_index_capacity: 40000,
        ..Default::default()
    }
}
pub async fn run() {
    let renderer = Renderer::new().expect("No se pudieron compilar los shaders GLSL");
    let catalog = bodies::catalog();
    let mut orbit = Orbit::new(20.);
    let mut selected = None;
    let mut paused = false;
    let mut time = 0.;
    let automatic = std::env::args().any(|a| a == "--capture-all");
    let mut frame = 0;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        let keys = [
            KeyCode::Key1,
            KeyCode::Key2,
            KeyCode::Key3,
            KeyCode::Key4,
            KeyCode::Key5,
            KeyCode::Key6,
        ];
        for (i, key) in keys.iter().enumerate() {
            if is_key_pressed(*key) {
                selected = Some(i);
                orbit = Orbit::new(6.5);
            }
        }
        if is_key_pressed(KeyCode::Key0) {
            selected = None;
            orbit = Orbit::new(20.);
        }
        if is_key_pressed(KeyCode::R) {
            orbit = Orbit::new(if selected.is_some() { 6.5 } else { 20. });
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if automatic {
            let view = frame / 8;
            selected = if view == 0 {
                None
            } else {
                Some((view - 1).min(5))
            };
            orbit = Orbit::new(if selected.is_some() { 6.5 } else { 20. });
        }
        orbit.update();
        if !paused {
            time += get_frame_time().min(0.1);
        }
        let target = selected.map_or(vec3(-0.8, 0., 0.), |i| catalog[i].position);
        clear_background(Color::from_rgba(5, 8, 19, 255));
        // Deterministic geometry-only starfield, independent of surface shaders.
        for i in 0..180 {
            let x = ((i * 7919) % 1440) as f32 / 1440. * screen_width();
            let y = ((i * 3571) % 900) as f32 / 900. * screen_height();
            draw_circle(
                x,
                y,
                if i % 7 == 0 { 1.2 } else { 0.6 },
                Color::from_rgba(105, 125, 155, 180),
            );
        }
        set_camera(&orbit.camera(target));
        renderer.draw(&catalog, time, orbit.eye(target), selected);
        set_default_camera();
        draw_rectangle(0., 0., screen_width(), 90., Color::from_rgba(5, 8, 19, 235));
        draw_text(
            "ATLAS / LABORATORIO 05",
            28.,
            32.,
            28.,
            Color::from_rgba(240, 196, 112, 255),
        );
        let name = selected.map_or("SEIS MUNDOS PROCEDURALES", |i| catalog[i].name);
        draw_text(name, 28., 64., 22., WHITE);
        draw_text(
            format!(
                "{} FPS  |  {}",
                get_fps(),
                if paused { "PAUSA" } else { "EN MOVIMIENTO" }
            ),
            screen_width() - 280.,
            35.,
            19.,
            LIGHTGRAY,
        );
        draw_rectangle(
            0.,
            screen_height() - 44.,
            screen_width(),
            44.,
            Color::from_rgba(5, 8, 19, 235),
        );
        draw_text("1-6 Seleccionar  /  0 Atlas  /  Arrastrar: orbita  /  Rueda: zoom  /  R Reiniciar  /  ESPACIO Pausa  /  F12 Captura",20.,screen_height()-17.,18.,LIGHTGRAY);
        if selected.is_none() {
            for b in &catalog {
                let uv = orbit
                    .camera(target)
                    .matrix()
                    .project_point3(b.position + vec3(0., -b.radius - 0.35, 0.));
                draw_text(
                    b.name,
                    (uv.x + 1.) * 0.5 * screen_width() - 35.,
                    (1. - uv.y) * 0.5 * screen_height(),
                    18.,
                    LIGHTGRAY,
                );
            }
        }
        if is_key_pressed(KeyCode::F12) || (automatic && frame % 8 == 7) {
            let name = selected.map_or("atlas", |i| catalog[i].name);
            if let Err(e) = std::fs::create_dir_all("docs/screenshots") {
                eprintln!("Captura: {e}");
            } else {
                get_screen_data().export_png(&format!("docs/screenshots/{name}.png"));
            }
        }
        frame += 1;
        if automatic && frame >= 56 {
            break;
        }
        next_frame().await;
    }
}
