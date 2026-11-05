use crate::{bodies, geometry, shaders};
use macroquad::prelude::*;
pub struct Renderer {
    sphere: Mesh,
    annulus: Mesh,
    body: Material,
    ring: Material,
}
impl Renderer {
    pub fn new() -> Result<Self, macroquad::Error> {
        Ok(Self {
            sphere: geometry::sphere(96, 64),
            annulus: geometry::rings(256),
            body: shaders::body()?,
            ring: shaders::rings()?,
        })
    }
    fn place(material: &Material, center: Vec3, radius: f32, spin: f32, tilt: f32) {
        material.set_uniform("Center", center);
        material.set_uniform("Radius", radius);
        material.set_uniform("Spin", spin);
        material.set_uniform("Tilt", tilt);
    }
    pub fn draw(&self, catalog: &[bodies::Body; 7], time: f32, eye: Vec3, selected: Option<usize>) {
        self.body.set_uniform("Time", time);
        self.body.set_uniform("Eye", eye);
        for (i, b) in catalog.iter().enumerate() {
            if selected.is_some_and(|index| index != i && !(index == 1 && i == 6)) {
                continue;
            }
            self.body.set_uniform("Kind", i as f32);
            Self::place(
                &self.body,
                bodies::position(catalog, i, time),
                b.radius,
                time * b.speed,
                0.,
            );
            gl_use_material(&self.body);
            draw_mesh(&self.sphere);
        }
        if selected.is_none() || selected == Some(2) {
            Self::place(&self.ring, catalog[2].position, catalog[2].radius, 0., 0.38);
            gl_use_material(&self.ring);
            draw_mesh(&self.annulus);
        }
        gl_use_default_material();
    }
}
