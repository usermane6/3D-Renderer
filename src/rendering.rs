use pixels::{Pixels, Error};
use super::states::State;

pub struct Renderer {
    pixels: Pixels
}

impl Renderer {
    pub fn new(pixels: Pixels) -> Self {
        Self { pixels }
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