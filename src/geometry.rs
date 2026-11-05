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
            let p = vec3(phi.sin()*theta.cos(), phi.cos(), phi.sin()*theta.sin());
            vertices.push(Vertex::new(p.x,p.y,p.z,u,v,WHITE));
        }
    }
    for y in 0..stacks { for x in 0..slices {
        let a = y*(slices+1)+x; let b = a+slices+1;
        indices.extend_from_slice(&[a,a+1,b,a+1,b+1,b]);
    }}
    Mesh { vertices, indices, texture: None }
}
pub fn rings(segments: u16) -> Mesh {
    assert!(segments >= 3 && segments < 32767);
    let mut vertices=Vec::new(); let mut indices=Vec::new();
    for i in 0..=segments {
        let a=i as f32/segments as f32*std::f32::consts::TAU;
        for r in [1.35,2.15] {
            vertices.push(Vertex::new(r*a.cos(),0.0,r*a.sin(),0.0,0.0,WHITE));
        }
    }
    for i in 0..segments { let a=i*2; indices.extend_from_slice(&[a,a+2,a+1,a+1,a+2,a+3]); }
    Mesh{vertices,indices,texture:None}
}
