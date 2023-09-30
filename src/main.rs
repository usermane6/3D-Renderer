// #![allow(unused_imports)]
use math::Vec2;
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

    let rads = ((render_data.total_updates)  as f64).to_radians();
    // let rads:f64 = 1.;
    let end = (Vec2::new(rads.sin(), rads.cos()) * 100.) + Vec2::new(250., 250.);
    // println!("{end}");
    drawing::line(&mut state, Vec2::new(250., 250.), end, &color::Color::BLUE());

    state
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    rendering::run_loop(renderer, event_loop, &redraw)    
}