#![allow(unused_imports)]
use math::Vec2;
use pixels::{Error, Pixels, SurfaceTexture};
use rendering::Renderer;
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

const WIDTH:  u32 = 500;
const HEIGHT: u32 = 500;

fn redraw(renderer: &mut Renderer, range: &mut Range<usize>) {
    let state = if let Some(id) = range.next() {
        let mut state = states::State::new_fill((WIDTH as usize, HEIGHT as usize), 0x00, 0x00, 0x00, 0xff);
        drawing::bar(&mut state, id);
        state
    } else {
        states::State::new_fill((WIDTH as usize, HEIGHT as usize), 0x00, 0x00, 0x00, 0xff)
    };

    renderer.update_buffer(state);
}

fn main() -> Result<(), Error> {
    // env::set_var("RUST_BACKTRACE", "1");
    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();

    let window = {
        let size = PhysicalSize::new(WIDTH, HEIGHT);

        WindowBuilder::new()
            .with_title("Test")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(
            window_size.width,
            window_size.height,
            &window
        );

        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };   

    let mut renderer = rendering::Renderer::new(pixels);

    let mut state = states::State::new_fill((WIDTH as usize, HEIGHT as usize), 0x00, 0x00, 0x00, 0xff);

    // renderer.update_buffer(state.clone());
    let mut range = 0..HEIGHT as usize;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            redraw(&mut renderer, &mut range);

            if let Err(_err) = renderer.render() {
                // error!("pixels.render() failed: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        
        if input.update(&event) {
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        window.request_redraw();
    });
}