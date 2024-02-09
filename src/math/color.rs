#[allow(unused)]
pub enum Colors {
    RED,
    GREEN,
    BLUE,
    MAGENTA,
    CYAN,
    YELLOW,
    WHITE,
    BLACK,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new( r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 0xff }
    }

    pub fn new_a( r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn new_color(color: Colors) -> Self {
        match color {
            Colors::RED =>     Color::new(0xff, 0x00, 0x00),
            Colors::GREEN =>   Color::new(0x00, 0xff, 0x00),
            Colors::BLUE =>    Color::new(0x00, 0x00, 0xff),
            Colors::MAGENTA => Color::new(0xff, 0x00, 0xff),
            Colors::CYAN =>    Color::new(0x00, 0xff, 0xff),
            Colors::YELLOW =>  Color::new(0xff, 0xff, 0x00),
            Colors::WHITE =>   Color::new(0xff, 0xff, 0xff),
            Colors::BLACK =>   Color::new(0x00, 0x00, 0x00),
            // _             =>   Color::new(0x00, 0x00, 0x00)
        }
    }

    pub fn gray(n: u8) -> Self {
        Color::new(n, n, n)
    }
}