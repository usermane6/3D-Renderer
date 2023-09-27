pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new( r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 0xff }
    }

    pub fn new_a( r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn RED() -> Self {
        Color::new(0xff, 0x00, 0x00)
    }

    pub fn GREEN() -> Self {
        Color::new(0x00, 0xff, 0x00)
    }

    pub fn BLUE() -> Self {
        Color::new(0x00, 0x00, 0xff)
    }

    pub fn BLACK() -> Self {
        Color::new(0x00, 0x00, 0x00)
    }

    pub fn GRAY(n: u8) -> Self {
        Color::new(n, n, n)
    }
}