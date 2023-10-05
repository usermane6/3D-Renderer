// #![allow(unused_imports)]
use math::{Vec2, Tri2d};
use rendering::RenderData;
use states::State;
use winit::event_loop::EventLoop;

mod rendering;
mod states;
mod drawing;
mod math;
mod color;

const WIDTH:  u32 = 500;
const HEIGHT: u32 = 500;

fn redraw(render_data: RenderData) -> states::State {
    let mut state = State::new_fill((WIDTH as usize, HEIGHT as usize), 0x00, 0x00, 0x00, 0xff);

    let a = Vec2::new(100., 100.,);
    let b = Vec2::new(200., 100.,);
    let c = Vec2::new(200., 200.,);
    let d = Vec2::new(100., 200.,);

    let a_tri = [a, b, c];
    let b_tri = [d, c, a];

    drawing::tri_filled(&mut state, &a_tri, &color::Color::RED());
    drawing::tri_filled(&mut state, &b_tri, &color::Color::RED());

    state   
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    rendering::run_loop(renderer, event_loop, &redraw)    
}