// #![allow(unused_imports)]
use math::{vec2::Vec2, vec3::Vec3};
use rendering::RenderData;
use states::State;
use winit::event_loop::EventLoop;

mod rendering;
mod states;
mod drawing;
mod math;
mod color;

const WIDTH:  u32 = 1000;
const HEIGHT: u32 = 1000;

fn redraw(render_data: RenderData) -> states::State {
    let mut state = State::new_fill((WIDTH as usize, HEIGHT as usize), color::Color::new(0x00, 0x00, 0x00));

    let a = Vec2::new(100., 100.,);
    let b = Vec2::new(200., 100.,);
    let c = Vec2::new(200., 200.,);
    let d = Vec2::new(100., 200.,);

    let a_tri = [a, b, c];
    let b_tri = [d, c, a];

    drawing::tri_filled(&mut state, &a_tri, &color::Color::new_color( color::Colors::RED ));
    drawing::tri_filled(&mut state, &b_tri, &color::Color::new_color( color::Colors::BLUE ));

    state   
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    rendering::run_loop(renderer, event_loop, &redraw)    
}