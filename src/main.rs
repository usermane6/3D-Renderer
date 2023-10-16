// #![allow(unused_imports)]
use math::{vec2::Vec2, vec3::Vec3, traits::VecMath, mat4::Mat4};
use object3d::Cube;
use rendering::RenderData;
use state2d::State;
use winit::event_loop::EventLoop;

mod rendering;
mod state2d;
mod drawing;
mod math;
mod color;
mod scene3d;
mod object3d;

const WIDTH:  u32 = 800;
const HEIGHT: u32 = 800;

//TODO define tri2d/3d as struct with points and also a color

fn redraw(render_data: RenderData) -> state2d::State {
    let mut state = State::new_fill((WIDTH as usize, HEIGHT as usize), color::Color::new(0x00, 0x00, 0x00));

    let a = Vec2::new(100., 100.,);
    let b = Vec2::new(200., 100.,);
    let c = Vec2::new(200., 200.,);
    let d = Vec2::new(100., 200.,);

    let a_tri = [a, b, c];
    let b_tri = [d, c, a];

    drawing::tri_filled(&mut state, &a_tri, &color::Color::new_color( color::Colors::RED ));
    drawing::tri_filled(&mut state, &b_tri, &color::Color::new_color( color::Colors::BLUE ));

    state   
}

fn test_3d(render_data: RenderData) -> state2d::State {
    // print!("fetching state");
    let mut scene = scene3d::Scene::new(vec![], scene3d::Viewport::new(1., 1., 1.), (WIDTH as usize, HEIGHT as usize));
    let a = Vec3::new(1., 2., 10.);
    let b = Vec3::new(-2., -2., 15.);
    let t = (render_data.total_updates as f64 * 2.).to_radians().sin().abs() * 2.;
    let translation = a.lerp(&b, t);
    let translation1 = a.lerp(&b, 1. - t);

    let mut cube = Cube::new().translate(translation);
    // let mut cube1 = Cube::new().translate(translation1);

    scene.append_mesh(&mut cube.mesh);
    // scene.append_mesh(&mut cube1.mesh);
    
    scene.get_state()
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    // let event_loop = EventLoop::new();

    // let renderer = rendering::Renderer::new(&event_loop, WIDTH, HEIGHT);

    // rendering::run_loop(renderer, event_loop, &test_3d)    

    let a = Mat4::scale(4.);
    let b = Mat4::scale(2.);

    println!("{:?}", a * b);
}