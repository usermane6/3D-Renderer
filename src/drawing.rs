use crate::math::color::Color;
use crate::state2d::State;
use crate::math::{vec2::Vec2, tri2d::Tri2d};

#[allow(unused)]
pub fn bar(s: &mut State, y: &usize, start_x: &usize, end_x: &usize, color: &Color) {
    for id in (y*s.width() + start_x)..(y*s.width() + end_x) {
        s.put_pixel_id(id, &color)
    }
}

/// interpolate function from this
/// https://gabrielgambetta.com/computer-graphics-from-scratch/06-lines.html
fn y_values(start: Vec2, end: Vec2) -> Vec<usize> {
    let mut ys = vec![];
    // let ys: Vec<usize> = vec![];

    // let dx = (start.x - end.x) as usize;
    // let dy = (start.y - end.y) as usize;

    let slope = (end.y - start.y) / (end.x - start.x); 
    let mut y = start.y;

    for _ in start.x as usize..end.x as usize {
        ys.push(y as usize);
        y += slope;
    }

    ys
}

fn x_values(start: Vec2, end: Vec2) -> Vec<usize> {
    y_values(start.swap_xy(), end.swap_xy())
}

fn line_gradual(s: &mut State, a: Vec2, b: Vec2, color: &Color) {
    // swap a/b depending on which point is farther on the x axis
    let (start, end) = if a.x > b.x { (b, a) } else { (a, b) };
    
    let ys = y_values(start, end);

    for x in start.x as usize..end.x as usize {
        s.put_pixel_xy(x, ys[x - start.x as usize], color)
    }
}

fn line_steep(s: &mut State, a: Vec2, b: Vec2, color: &Color) {
    // swap a/b depending on which point is farther on the x axis
    let (start, end) = if a.y > b.y { (b, a) } else { (a, b) };

    let xs = x_values(start, end);

    for y in start.y as usize..end.y as usize {
        s.put_pixel_xy(xs[y - start.y as usize], y, color)
    }
}

/// draws a line to the screen from start to end
///- a and b can be in any order
pub fn line(s: &mut State, a: Vec2, b: Vec2, color: &Color) {
    if (a.x - b.x).abs() > (a.y - b.y).abs() { // determine if you need to iterate in y or x
        line_gradual(s, a, b, color);
    } else {
        line_steep(s, a, b, color);
    }
}

pub fn tri_wireframe(s: &mut State, tri: &Tri2d, color: &Color) {
    line(s, tri[0], tri[1], color);
    line(s, tri[1], tri[2], color);
    line(s, tri[2], tri[0], color);
}

#[allow(unused)]
pub fn tri_filled(s: &mut State, tri: &Tri2d, color: &Color) {
    //! god strike me down i cant do this anymore
    let mut height_order = [tri[0], tri[1], tri[2]];
    
    // sort all points by height: highest -> lowest
    if height_order[1].y <= height_order[0].y { height_order.swap(1, 0) }
    if height_order[2].y <= height_order[0].y { height_order.swap(2, 0) }
    if height_order[2].y <= height_order[1].y { height_order.swap(2, 1) }

    let x_012 = {
        let mut x_01 = x_values(height_order[0], height_order[1]);
        let mut x_12 = x_values(height_order[1], height_order[2]);
        // x_01.pop();
        x_01.append(&mut x_12);
        x_01
    };
    let x_02 = x_values(height_order[0], height_order[2]);

    let is_mid_left =  height_order[1].x < height_order[2].x;

    if height_order[2].y <= 0. {return}

    if is_mid_left {
        for y in height_order[0].y as usize..=(height_order[2].y as usize - 1) {
            let i = y - height_order[0].y  as usize;
            // print!("{} ", i);
            for x in x_012[i]..=x_02[i] {
                s.put_pixel_xy(x, y, color)
            }
        }
    } else {
        for y in height_order[0].y as usize..=(height_order[2].y as usize - 1) {
            let i = y - height_order[0].y  as usize;
            // print!("{} ", i);
            for x in x_02[i]..=x_012[i] {
                s.put_pixel_xy(x, y, color)
            }
        }
    }

}