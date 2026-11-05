//! Indexed, outward-facing unit sphere and independent annulus, without image textures.
use macroquad::prelude::*;
pub fn sphere(slices: u16, stacks: u16) -> Mesh {
    assert!(slices >= 3 && stacks >= 2);
    assert!((slices as usize + 1) * (stacks as usize + 1) <= u16::MAX as usize);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for y in 0..=stacks {
        let v = y as f32 / stacks as f32;
        let phi = v * std::f32::consts::PI;
        for x in 0..=slices {
            let u = x as f32 / slices as f32;
            let theta = u * std::f32::consts::TAU;
            let p = vec3(phi.sin() * theta.cos(), phi.cos(), phi.sin() * theta.sin());
            vertices.push(Vertex::new(p.x, p.y, p.z, u, v, WHITE));
        }
    }
    for y in 0..stacks {
        for x in 0..slices {
            let a = y * (slices + 1) + x;
            let b = a + slices + 1;
            indices.extend_from_slice(&[a, a + 1, b, a + 1, b + 1, b]);
        }
    }
    Mesh {
        vertices,
        indices,
        texture: None,
    }
}
pub fn rings(segments: u16) -> Mesh {
    assert!((3..32767).contains(&segments));
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for i in 0..=segments {
        let a = i as f32 / segments as f32 * std::f32::consts::TAU;
        for r in [1.35, 2.15] {
            vertices.push(Vertex::new(r * a.cos(), 0.0, r * a.sin(), 0.0, 0.0, WHITE));
        }
    }
    for i in 0..segments {
        let a = i * 2;
        indices.extend_from_slice(&[a, a + 2, a + 1, a + 1, a + 2, a + 3]);
    }
    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sphere_indices_and_normals_are_valid() {
        let mesh = sphere(32, 16);
        assert_eq!(mesh.vertices.len(), 33 * 17);
        assert_eq!(mesh.indices.len(), 32 * 16 * 6);
        for v in &mesh.vertices {
            assert!((v.position.length() - 1.).abs() < 1e-5);
        }
        let mut triangles = 0;
        for t in mesh.indices.as_chunks::<3>().0 {
            let a = mesh.vertices[t[0] as usize].position;
            let b = mesh.vertices[t[1] as usize].position;
            let c = mesh.vertices[t[2] as usize].position;
            let normal = (b - a).cross(c - a);
            if normal.length() > 1e-6 {
                assert!(normal.dot(a + b + c) > 0.);
                triangles += 1;
            }
        }
        assert_eq!(triangles, 32 * 30);
        assert!(mesh.texture.is_none());
    }
    #[test]
    fn annulus_is_separate_and_has_open_center() {
        let mesh = rings(128);
        assert_eq!(mesh.indices.len(), 128 * 6);
        for v in &mesh.vertices {
            assert!(v.position.y.abs() < 1e-6);
            assert!(v.position.length() >= 1.349);
            assert!(v.position.length() <= 2.151);
        }
        assert!(mesh
            .indices
            .iter()
            .all(|i| (*i as usize) < mesh.vertices.len()));
        assert!(mesh.texture.is_none());
    }
    #[test]
    #[should_panic]
    fn reject_mesh_index_overflow() {
        sphere(500, 500);
    }
}
