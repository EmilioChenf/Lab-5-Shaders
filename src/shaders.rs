//! Shader sources are embedded: binaries never depend on the current working directory.
use macroquad::prelude::*;
pub fn body() -> Result<Material, macroquad::Error> {
    load_material(ShaderSource::Glsl {
        vertex: include_str!("../shaders/body.vert"),
        fragment: include_str!("../shaders/body.frag"),
    }, MaterialParams {
        pipeline_params: PipelineParams { depth_test: Comparison::LessOrEqual, depth_write: true, ..Default::default() },
        uniforms: vec![UniformDesc::new("Time",UniformType::Float1),UniformDesc::new("Kind",UniformType::Float1),UniformDesc::new("Spin",UniformType::Float1),UniformDesc::new("Center",UniformType::Float3),UniformDesc::new("Radius",UniformType::Float1),UniformDesc::new("Tilt",UniformType::Float1),UniformDesc::new("Eye",UniformType::Float3)],
        ..Default::default()
    })
}
pub fn rings() -> Result<Material, macroquad::Error> {
    load_material(ShaderSource::Glsl {
        vertex: include_str!("../shaders/body.vert"),
        fragment: include_str!("../shaders/rings.frag"),
    }, MaterialParams {
        pipeline_params: PipelineParams { depth_test: Comparison::LessOrEqual, depth_write: true, ..Default::default() },
        uniforms: vec![UniformDesc::new("Spin",UniformType::Float1),UniformDesc::new("Center",UniformType::Float3),UniformDesc::new("Radius",UniformType::Float1),UniformDesc::new("Tilt",UniformType::Float1)],
        ..Default::default()
    })
}
