// #![allow(unused_imports)]
use math::{vec3::Vec3, vec4::Vec4};
use object3d::Object3d;
// use object3d::Cube;
use rendering::RenderData;
use scene3d::Scene;
use winit::event_loop::EventLoop;

mod rendering;
mod state2d;
mod drawing;
mod math;
mod scene3d;
mod object3d;

const WIDTH:  u32 = 800;
const HEIGHT: u32 = 800;

fn on_start(scene: &mut Scene, _: RenderData) {
    let mut object = Object3d::plane();

    object.transform.translation = Vec4::new(0., 0., 10., 1.);
    object.transform.rotation += Vec3::new(0., 0.1, 0.1);

    scene.add_object(object);
}

fn update(scene: &mut Scene, _render_data: RenderData) {
    if let Some(object) = scene.objects.get_mut(0) {
        object.transform.rotation += Vec3::new(0., 0.02, 0.);
    }
}

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let event_loop = EventLoop::new();

    let renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    rendering::run_loop(renderer, event_loop, &on_start, &update)    
}