use macroquad::prelude::*;
/// Orbit state is independent of simulation time, so inspection works while paused.
pub struct Orbit {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}
impl Orbit {
    pub fn new(distance: f32) -> Self {
        Self {
            yaw: 0.,
            pitch: 0.32,
            distance,
        }
    }
    pub fn update(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let d = mouse_delta_position();
            self.yaw -= d.x * 3.;
            self.pitch = (self.pitch + d.y * 3.).clamp(-1.35, 1.35);
        }
        self.distance = (self.distance * (-mouse_wheel().1 * 0.12).exp()).clamp(2.5, 65.);
    }
    pub fn eye(&self, target: Vec3) -> Vec3 {
        target
            + vec3(
                self.yaw.sin() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.cos() * self.pitch.cos(),
            ) * self.distance
    }
    pub fn camera(&self, target: Vec3) -> Camera3D {
        Camera3D {
            position: self.eye(target),
            target,
            up: Vec3::Y,
            fovy: 45.0_f32.to_radians(),
            z_near: 0.05,
            z_far: 150.,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn orbit_preserves_distance_and_targets_translation() {
        let orbit = Orbit::new(6.5);
        let target = vec3(4., 2., -3.);
        assert!((orbit.eye(target).distance(target) - 6.5).abs() < 1e-5);
        assert!((orbit.eye(target) - orbit.eye(Vec3::ZERO) - target).length() < 1e-5);
        assert!(orbit.camera(target).z_near < 1.);
    }
}
