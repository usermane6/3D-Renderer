use crate::color::Color;

#[derive(Clone)]
pub struct State {
    pub pixels: Vec<u8>,
    pub size: (usize, usize)
}

impl State {
    pub fn width(&self)  -> usize { self.size.0 }
    pub fn height(&self) -> usize { self.size.1 }

    pub fn new_fill_raw( size: (usize, usize), r: u8, g:u8, b:u8, a:u8 ) -> Self {

        let mut pixels = vec![];

        for _ in 0..size.0 * size.1 {
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(a);
        }

        Self { pixels, size }
    }

    pub fn new_fill( size: (usize, usize), c: Color) -> Self {
        State::new_fill_raw(size, c.r, c.g, c.b, c.a)
    }

    pub fn put_pixel_xy_raw(&mut self, x: usize, y: usize, r: u8, g:u8, b:u8, a:u8 ) {
        let id = ((self.size.0) * y) + x;

        self.put_pixel_id_raw(id, r, g, b, a);
    }

    pub fn put_pixel_id_raw(&mut self, id:usize, r: u8, g:u8, b:u8, a:u8) {
        let id = id * 4; // each pixel has 4 values in the list
        if let None = self.pixels.get(id) { panic!("Out of Bounds Error: Could not place pixel at ({id})") }

        self.pixels[id + 0] = r;
        self.pixels[id + 1] = g;
        self.pixels[id + 2] = b;
        self.pixels[id + 3] = a;
    }

    pub fn put_pixel_id(&mut self, id:usize, c: &Color) {
        self.put_pixel_id_raw(id, c.r, c.g, c.b, c.a);
    }

    pub fn put_pixel_xy(&mut self, x:usize, y: usize, c: &Color) {
        self.put_pixel_xy_raw(x, y, c.r, c.g, c.b, c.a);
    }
}

/// sets the default settings for states.
/// used for convenient and quick creation of new states
pub struct StateBuilder {
    w: usize,
    h: usize,
    clear_color: Color
}

impl StateBuilder {
    fn new(w: usize, h: usize, clear_color: Color) -> Self {
        StateBuilder{ w, h, clear_color }
    }

    fn new_state(&self) -> State {
        State::new_fill( (self.w, self.h), self.clear_color)
    }
}