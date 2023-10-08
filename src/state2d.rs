use crate::color::Color;

#[derive(Clone)]
pub struct State {
    pub size: (usize, usize),
    pub pixels: Vec<u8>,
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
        if !self.is_in_bounds(x, y) { return }

        let id = ((self.size.0) * y) + x;

        self.put_pixel_id_raw(id, r, g, b, a);
    }

    pub fn put_pixel_id_raw(&mut self, id:usize, r: u8, g:u8, b:u8, a:u8) {
        let id = id * 4; // each pixel has 4 values in the list
        // if let None = self.pixels.get(id) { panic!("Out of Bounds Error: Could not place pixel at ({id})") }
        if let None = self.pixels.get(id) { return }

        self.pixels[id + 0] = r;
        self.pixels[id + 1] = g;
        self.pixels[id + 2] = b;
        self.pixels[id + 3] = a;
    }

    pub fn put_pixel_id(&mut self, id:usize, c: &Color) {
        self.put_pixel_id_raw(id, c.r, c.g, c.b, c.a);
    }

    pub fn put_pixel_xy(&mut self, x:usize, y: usize, c: &Color) {

        // println!("{x}, {y}");
        self.put_pixel_xy_raw(x, y, c.r, c.g, c.b, c.a);
    }

    pub fn is_in_bounds( &self, x: usize, y: usize ) -> bool {
        if x > self.width()  { return false; }
        if y > self.height() { return false; }

        true
    }
}