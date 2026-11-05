//! Shader sources are embedded: binaries never depend on the current working directory.
use macroquad::prelude::*;
pub fn body() -> Result<Material, macroquad::Error> {
    load_material(ShaderSource::Glsl {
        vertex: include_str!("../shaders/body.vert"),
        fragment: include_str!("../shaders/body.frag"),
    }, MaterialParams {
        pipeline_params: PipelineParams { depth_test: Comparison::LessOrEqual, depth_write: true, ..Default::default() },
        uniforms: vec![UniformDesc::new("Time",UniformType::Float1),UniformDesc::new("Kind",UniformType::Float1),UniformDesc::new("Spin",UniformType::Float1),UniformDesc::new("Eye",UniformType::Float3)],
        ..Default::default()
    })
}
