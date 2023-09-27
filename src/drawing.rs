use super::states::State;
use super::math::Vec2;

pub fn bar(s: &mut State, y: usize) {
    for id in (y*s.width())..(y*s.width()+s.width()) {
        s.put_pixel_id(id, 0xff, 0xff, 0x00, 0xff)
    }
}

pub fn line(s: &mut State, p1: Vec2, p2: Vec2) {
    let slope = (p1.y - p2.y) / (p1.x - p2.x);

    let range = {    
        let (range_start, range_end) = if p1.x > p1.x { (p2.x, p1.x) } else { (p1.x, p2.x) };
        range_start as usize .. range_end as usize
    };

    for x in range {
        let y = (slope * (x as f64 - p1.x) + p1.y) as usize; // point slope form
        s.put_pixel_xy(x, y, 0xff, 0xff, 0x00, 0xff);
    }
}