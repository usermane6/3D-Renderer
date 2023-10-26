use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use winit_input_helper::WinitInputHelper;

use crate::math::tri3d::Tri3d;
use crate::scene3d::Scene;

use super::state2d::State;

pub struct Renderer {
    pixels: Pixels,
    pub(crate) scene: Scene,

    window: Window,
    input: WinitInputHelper,
}

impl Renderer {
    pub fn new(event_loop: &EventLoop<()>, w: u32, h: u32) -> Self {
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

        let scene = Scene::new_empty((w as usize, h as usize));

        if let Ok(pixels) = pixels_result {
            Self {
                pixels,
                scene,
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
        for (frame_p, state_p) in zipped { 
            if *frame_p == state_p {continue;}
            *frame_p = state_p; 
        }
    }   

    pub fn render(&mut self) -> Result<(), Error> {
        self.pixels.render()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RenderData {
    pub total_updates: u32,
}

pub fn run_loop<'a, F, G>(mut renderer: Renderer, event_loop: EventLoop<()>, on_start: F, update: G) 
    where 
    F: Fn(&mut Scene) + 'static,
    G: Fn(&mut Scene) + 'static 
{
    let mut render_data = RenderData { 
        total_updates: 0,
    };

    on_start(&mut renderer.scene);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // redraw has been requeated => get new state => update pixel buffer
            update(&mut renderer.scene);
            renderer.update_buffer(renderer.scene.get_state());
            render_data.total_updates += 1;
            
            // panic!();
            if let Err(_err) = renderer.render() {
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