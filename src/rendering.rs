use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop, self};
use winit::window::{WindowBuilder, Window};
use winit_input_helper::WinitInputHelper;

use super::states::State;

pub struct Renderer {
    pixels: Pixels,

    window: Window,
    input: WinitInputHelper,
}

impl Renderer {
    pub fn new(event_loop: &EventLoop<()>,w: u32, h: u32) -> Self {
        let input = WinitInputHelper::new();

        let window = {
            let size = PhysicalSize::new(w, h);

            WindowBuilder::new()
                .with_title("Test")
                .with_inner_size(size)
                .with_resizable(false)
                .build(event_loop)
                .unwrap()
        };

        let pixels_result = {

            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(
                window_size.width,
                window_size.height,
                &window
            );

            Pixels::new(w, h, surface_texture)
        };

        if let Ok(pixels) = pixels_result {
            Self {
                pixels,
                window,
                input,
            }
        } else {
            panic!("Error creating Pixel buffer")
        }
    }

    pub fn update_buffer(&mut self, state: State) {
        //todo: make this not sloppy
        let zipped = self.pixels.frame_mut().iter_mut().zip(state.pixels);
        for (frame_p, state_p) in zipped { *frame_p = state_p }
    }   

    pub fn render(&mut self) -> Result<(), Error> {
        self.pixels.render()
    }
}

pub struct RenderData {
    total_updates: u32,
}

pub fn run_loop(mut renderer: Renderer, event_loop: EventLoop<()>, next_state: &'static dyn Fn() -> State) {
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            renderer.update_buffer(next_state());

            if let Err(_err) = renderer.render() {
                // error!("pixels.render() failed: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        
        if renderer.input.update(&event) {
            if renderer.input.close_requested() || renderer.input.destroyed() {
                *control_flow = ControlFlow::Exit;
                return;
            }   
        }

        renderer.window.request_redraw();
    });
}