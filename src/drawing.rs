use std::collections::HashSet;

use crate::color::Color;
use crate::states::State;
use crate::math::{Vec2, Tri2d};

// TODO: have drawing functions return states, rather than modify them
// TODO TODO: cleanup
// // TODO: use color as a param 

pub fn bar(s: &mut State, y: &usize, start_x: &usize, end_x: &usize, color: &Color) {
    for id in (y*s.width() + start_x)..(y*s.width() + end_x) {
        s.put_pixel_id(id, &color)
    }
}

fn line_gradual(s: &mut State, a: Vec2, b: Vec2, color: &Color) {
    // swap a/b depending on which point is farther on the x axis
    let (start, end) = if a.x > b.x { (b, a) } else { (a, b) };
        
    for (x, y) in points_on_line(start, end) { 
        s.put_pixel_xy(x, y, color) 
    }
}

fn line_steep(s: &mut State, a: Vec2, b: Vec2, color: &Color) {
    // swap a/b depending on which point is farther on the x axis
    let (start, end) = if a.y > b.y { (b, a) } else { (a, b) };
    
    // xy must be swapped because iteration is in y
    // for (y, x) in points_on_line(start.swap_xy(), end.swap_xy()) { 
    //     s.put_pixel_xy(x, y, color) 
    // }

    let xs = x_values(start, end);

    for y in start.y as usize..end.y as usize {
        s.put_pixel_xy(xs[y - start.y as usize], y, color)
    }
}

/// Returns the pixel coords of all points on a line
///- start and end params MUST be in the correct order
///- the state must have have the lesser x value
fn points_on_line(start: Vec2, end: Vec2) -> Vec<(usize, usize)>{
    let mut points = vec![];

    let slope = (start.y - end.y) / (start.x - end.x);
    let mut x = start.x;
    
    for point_x in start.x as usize..end.x as usize {
        let point_y = (slope * (point_x as f64 - start.x) + start.y) as usize;

        points.push( (point_x, point_y) );
    }

    points
}

fn points_with_xy_swap(start: Vec2, end: Vec2) -> Vec<(usize, usize)> {
    let mut endpoints = [ start, end ];

    if (start.x - end.x).abs() > (start.y - end.y).abs() {
        if start.x > end.x { endpoints.swap(0, 1); };

        return points_on_line(endpoints[0], endpoints[1])
    } else {
        if start.y > end.y { endpoints.swap(0, 1); };

        return points_on_line(endpoints[0].swap_xy(), endpoints[1].swap_xy())
            .into_iter()
            .map(|(y, x)| (x, y))
            .collect()
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

/// draws a line to the screen from start to end
///- a and b can be in any order
pub fn line(s: &mut State, a: Vec2, b: Vec2, color: &Color) {
    if (a.x - b.x).abs() > (a.y - b.y).abs() { // determine if you need to iterate in y or x
        line_gradual(s, a, b, color);
    } else {
        line_steep(s, a, b, color);
    }
}

pub fn tri_wireframe(s: &mut State, tri: &Tri2d, color: &Color) 
{
    line(s, tri[0], tri[1], color);
    line(s, tri[1], tri[2], color);
    line(s, tri[2], tri[0], color);
}



pub fn tri_filled(s: &mut State, tri: &Tri2d, color: &Color) {
    //! god strike me down i cant do this anymore
    // todo: right side case
    let mut height_order = [tri[0], tri[1], tri[2]];
    
    // sort all points by height: highest -> lowest
    if height_order[1].y <= height_order[0].y { height_order.swap(1, 0) }
    if height_order[2].y <= height_order[0].y { height_order.swap(2, 0) }
    if height_order[2].y <= height_order[1].y { height_order.swap(2, 1) }

    let x_012 = {
        let mut x_01 = x_values(height_order[0], height_order[1]);
        let mut x_12 = x_values(height_order[1], height_order[2]);
        x_01.pop();
        x_01.append(&mut x_12);
        x_01
    };
    let x_02 = x_values(height_order[0], height_order[2]);

    let is_mid_left =  height_order[1].x < height_order[2].x;

    if is_mid_left {
        for y in height_order[0].y as usize..(height_order[2].y as usize - 1) {
            let i = y - height_order[0].y  as usize;
            // print!("{} ", i);
            for x in x_012[i]..x_02[i] {
                s.put_pixel_xy(x, y, color)
            }
        }
    } else {
        for y in height_order[0].y as usize..(height_order[2].y as usize - 1) {
            let i = y - height_order[0].y  as usize;
            // print!("{} ", i);
            for x in x_02[i]..x_012[i] {
                s.put_pixel_xy(x, y, color)
            }
        }
    }

}