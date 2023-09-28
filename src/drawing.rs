use super::states::State;
use super::math::Vec2;

// TODO: have drawing functions return states, rather than modify them
// TODO: use color as a param 

pub fn bar(s: &mut State, y: usize) {
    for id in (y*s.width())..(y*s.width()+s.width()) {
        s.put_pixel_id(id, 0xff, 0xff, 0xff, 0xff)
    }
}

fn line_gradual(s: &mut State, start: Vec2, end: Vec2) {
    let slope = (start.y - end.y) / (start.x - end.x);

    let range = {    
        let (range_start, range_end) = if start.x > start.x { (end.x, start.x) } else { (start.x, end.x) };
        range_start as usize .. range_end as usize
    };

    for x in range {
        let y = (slope * (x as f64 - start.x) + start.y) as usize; // point slope form
        s.put_pixel_xy(x, y, 0xff, 0xff, 0xff, 0xff);
    }
}

fn line_steep(s: &mut State, start: Vec2, end: Vec2) {
    let slope = (start.x - end.x) / (start.y - end.y);

    let range = {    
        let (range_start, range_end) = if start.y > start.y { (end.y, start.y) } else { (start.y, end.y) };
        range_start as usize .. range_end as usize
    };

    for y in range {
        let x = (slope * (y as f64 - start.y) + start.x) as usize; // point slope form
        s.put_pixel_xy(x, y, 0xff, 0xff, 0xff, 0xff);
    }
}

pub fn line(s: &mut State, start: Vec2, end: Vec2) {
    if (start.x - end.x).abs() > (start.y - end.y) {
        line_steep(s, start, end);
    } else {
        line_gradual(s, start, end);
    }
}