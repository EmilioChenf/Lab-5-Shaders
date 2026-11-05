mod app;
mod bodies;
mod camera;
mod geometry;
mod render;
mod shaders;
use app::configuration;
#[macroquad::main(configuration)]
async fn main() {
    app::run().await;
}
