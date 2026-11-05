use macroquad::prelude::*;
pub struct Body {
    pub name: &'static str,
    pub position: Vec3,
    pub radius: f32,
    pub speed: f32,
}
pub fn catalog() -> [Body; 7] {
    [
        Body {
            name: "PAPATIN",
            position: vec3(-7., 1.8, 0.),
            radius: 1.65,
            speed: 0.12,
        },
        Body {
            name: "PAPATINSENIOR",
            position: vec3(-2.6, 1.8, 0.),
            radius: 1.,
            speed: 0.23,
        },
        Body {
            name: "MAMBO",
            position: vec3(3.1, 1.8, 0.),
            radius: 1.35,
            speed: 0.16,
        },
        Body {
            name: "CRYON",
            position: vec3(-6., -2.3, 0.),
            radius: 1.1,
            speed: 0.19,
        },
        Body {
            name: "CITOPA",
            position: vec3(-1., -2.3, 0.),
            radius: 1.15,
            speed: 0.14,
        },
        Body {
            name: "VERDANTIS",
            position: vec3(4.7, -2.3, 0.),
            radius: 1.2,
            speed: 0.21,
        },
        Body {
            name: "NYX",
            position: vec3(-2.6, 2.05, 1.75),
            radius: 0.29,
            speed: 0.2,
        },
    ]
}
/// Nyx is independent geometry and follows its parent; all other centers stay fixed.
pub fn position(catalog: &[Body; 7], index: usize, time: f32) -> Vec3 {
    if index == 6 {
        moon_position(catalog[1].position, time)
    } else {
        catalog[index].position
    }
}
pub fn focus_distance(selected: Option<usize>) -> f32 {
    match selected {
        None => 20.,
        Some(2) => 10.,
        Some(6) => 2.5,
        Some(_) => 6.5,
    }
}
pub fn moon_position(parent: Vec3, time: f32) -> Vec3 {
    parent + vec3((time * 0.45).cos() * 1.75, 0.25, (time * 0.45).sin() * 1.75)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moon_orbit_stays_outside_parent() {
        let parent = catalog()[1].position;
        for t in 0..200 {
            let p = moon_position(parent, t as f32 * 0.1);
            assert!(
                (p.distance(parent) - (1.75_f32.powi(2) + 0.25_f32.powi(2)).sqrt()).abs() < 1e-5
            );
            assert!(p.distance(parent) > catalog()[1].radius + 0.29);
        }
        assert!(moon_position(parent, 0.).distance(moon_position(parent, 2.)) > 0.5);
    }
    #[test]
    fn moon_selection_tracks_orbit_and_has_own_framing() {
        let bodies = catalog();
        for time in [0., 1., 8., 20.] {
            assert_eq!(
                position(&bodies, 6, time),
                moon_position(bodies[1].position, time)
            );
            for i in 0..6 {
                assert_eq!(position(&bodies, i, time), bodies[i].position);
            }
        }
        assert_eq!(focus_distance(None), 20.);
        assert!(focus_distance(Some(2)) > focus_distance(Some(1)));
        assert!(focus_distance(Some(6)) < focus_distance(Some(1)));
    }
    #[test]
    fn catalog_has_seven_distinct_bodies() {
        let bodies = catalog();
        assert_eq!(
            bodies.map(|b| b.name),
            [
                "PAPATIN",
                "PAPATINSENIOR",
                "MAMBO",
                "CRYON",
                "CITOPA",
                "VERDANTIS",
                "NYX"
            ]
        );
        let bodies = catalog();
        for (i, a) in bodies.iter().enumerate() {
            assert!(a.radius > 0. && a.speed > 0.);
            for b in &bodies[i + 1..] {
                assert_ne!(a.name, b.name);
                assert!(a.position.distance(b.position) > a.radius + b.radius);
            }
        }
    }
}
