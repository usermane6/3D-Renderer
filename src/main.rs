#![allow(unused_imports)]
use math::Vec2;
use pixels::{Error, Pixels, SurfaceTexture};
use rendering::Renderer;
use states::State;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop, self};
use winit::window::{WindowBuilder, self};
use winit_input_helper::WinitInputHelper;
use std::env;
use std::ops::Range;

mod rendering;
mod states;
mod drawing;
mod math;
mod color;

const WIDTH:  u32 = 500;
const HEIGHT: u32 = 500;

fn redraw() -> states::State {
    let mut state = State::new_fill((WIDTH as usize, HEIGHT as usize), 0x00, 0x00, 0x00, 0xff);

    drawing::line(&mut state, Vec2::new(100., 240.), Vec2::new(300., 20.));

    state
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new();

    let mut renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    let mut state = states::State::new_fill((WIDTH as usize, HEIGHT as usize), 0x00, 0x00, 0x00, 0xff);

    // renderer.update_buffer(state.clone());
    let mut range = 0..HEIGHT as usize;

    rendering::run_loop(renderer, event_loop, &redraw)    
}