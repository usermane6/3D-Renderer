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
fn on_start(scene: &mut Scene, _: RenderData) {
    let mut cube = Object3d::cube();
    cube.transform.translation = Vec4::new(0., 0., 10., 1.);
    cube.transform.scale = 0.5;

    let mut cube2 = Object3d::cube();
    cube2.transform.translation = Vec4::new(0., 0., 10., 1.);
    cube2.transform.scale = 0.5;
    
    scene.add_object(cube);
    scene.add_object(cube2)
}

fn update(scene: &mut Scene, render_data: RenderData) {
    if let Some(object) = scene.objects.get_mut(0) {
        // object.transform.translation.x += 0.2;
        // object.transform.translation.y += 0.2;
        // object.transform.translation.z += 0.1;
        object.transform.rotation.x += 0.05;
        object.transform.rotation.y += 0.02;
        object.transform.rotation.z += 0.01;
    }

    if let Some(object) = scene.objects.get_mut(1) {
        let theta = (render_data.total_updates as f64 * 2.).to_radians();
        
        object.transform.translation.x = theta.sin() * 4.;
        object.transform.translation.y = theta.cos() * 4.;
        // object.transform.translation.y += 0.2;
        // object.transform.translation.z += 0.1;
        // object.transform.rotation.x += 0.03;
        // object.transform.rotation.y += 0.04;
        // object.transform.rotation.z += 0.06;
    }
}

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    rendering::run_loop(renderer, event_loop, &on_start, &update)    
}