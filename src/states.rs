#[derive(Clone)]
pub struct State {
    pub pixels: Vec<u8>,
    pub size: (usize, usize)
}

impl State {
    pub fn width(&self) -> usize { self.size.0 }
    pub fn height(&self) -> usize { self.size.1 }

    pub fn new_fill( size: (usize, usize), r: u8, g:u8, b:u8, a:u8 ) -> Self {

        let mut pixels = vec![];

        for _ in 0..size.0 * size.1 {
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(a);
        }

        Self { pixels, size }
    }

    pub fn put_pixel_xy(&mut self, x: usize, y: usize, r: u8, g:u8, b:u8, a:u8 ) {
        let id = ((self.size.0) * y) + x;

        self.put_pixel_id(id, r, g, b, a);
    }

    pub fn put_pixel_id(& mut self, id:usize, r: u8, g:u8, b:u8, a:u8) {
        let id = id * 4;
        if let None = self.pixels.get(id) { panic!("Out of Bounds Error: Could not place pixel at ({id})") }

        self.pixels[id + 0] = r;
        self.pixels[id + 1] = g;
        self.pixels[id + 2] = b;
        self.pixels[id + 3] = a;
    }
}