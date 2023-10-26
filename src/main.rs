// #![allow(unused_imports)]
use math::{vec2::Vec2, vec3::Vec3, traits::VecMath, mat4::Mat4, vec4::Vec4};
use object3d::Object3d;
// use object3d::Cube;
use rendering::RenderData;
use scene3d::Scene;
use state2d::State;
use winit::event_loop::EventLoop;

mod rendering;
mod state2d;
mod drawing;
mod math;
mod scene3d;
mod object3d;

const WIDTH:  u32 = 800;
const HEIGHT: u32 = 800;

// TODO: impl addassign to all vector classes
fn on_start(scene: &mut Scene) {
    let mut cube = Object3d::cube();
    cube.transform.translation = Vec4::new(0., 0., 5., 1.);
    cube.transform.scale = 0.5;
    scene.add_object(cube);
}

fn update(scene: &mut Scene) {
    if let Some(object) = scene.objects.get_mut(0) {
        // object.transform.translation.x += 0.2;
        // object.transform.translation.y += 0.2;
        // object.transform.translation.z += 0.;
        object.transform.rotation += 0.05;
    }
}

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    rendering::run_loop(renderer, event_loop, &on_start, &update)    
}