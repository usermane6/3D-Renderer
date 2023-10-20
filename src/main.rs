// #![allow(unused_imports)]
use math::{vec2::Vec2, vec3::Vec3, traits::VecMath, mat4::Mat4};
// use object3d::Cube;
use rendering::RenderData;
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

//TODO define tri2d/3d as struct with points and also a color

fn redraw(render_data: RenderData) -> state2d::State {
    State::new_fill((WIDTH as usize, HEIGHT as usize), math::color::Color::gray(100))
}

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    rendering::run_loop(renderer, event_loop, &redraw)    
}