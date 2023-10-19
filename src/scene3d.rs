use crate::{math::{Tri3d, vec3::Vec3, Tri2d, vec2::Vec2, color::{Color, Colors}}, state2d::State, drawing};

/// plane in 3d space that all points are projected onto for rendering
pub struct Viewport {
    w: f64,
    h: f64,
    d: f64,
}

impl Viewport {
    pub fn new(w: f64, h:f64, d:f64) -> Self{
        Viewport {w, h, d}
    }

    pub fn project_onto(&self, p: &Vec3) -> Vec3 {
        Vec3 { 
            x: ( p.x * self.d ) / p.z, 
            y: ( p.y * self.d ) / p.z, 
            z: self.d
        }
    }
}

// TODO: connect this more heavily to Renderer class to take advantage of state builder
pub struct Scene {
    viewport: Viewport,
    state_size: (usize, usize),
    mesh: Vec<Tri3d>,
}

impl Scene {
    pub fn new(mesh: Vec<Tri3d>, viewport: Viewport, state_size: (usize, usize)) -> Self {
        Scene { mesh, viewport, state_size }
    }

    pub fn new_empty(state_size: (usize, usize)) -> Self {
        Scene { 
            viewport: Viewport::new(1., 1., 1.), 
            state_size: state_size, 
            mesh: vec![] 
        }
    }

    pub fn get_state(&self) -> State{
        let mut s = State::new_fill(self.state_size, Color::new_color(Colors::BLACK));

        let projected_mesh = self.project_mesh();

        // print!("drawing");
        for tri in projected_mesh {
            // print!("{:?} ", tri);
            drawing::tri_wireframe(&mut s, &tri, &Color::new_color(Colors::BLUE));
        }

        s
    }

    fn project_mesh(&self) -> Vec<Tri2d> {
        let mut projected_tris = vec![];

        for [a, b, c] in self.mesh.iter() {
            projected_tris.push([
                self.project_onto_2d(a),
                self.project_onto_2d(b),
                self.project_onto_2d(c),
            ])
        }

        projected_tris
    }

    fn project_onto_2d(&self, p: &Vec3) -> Vec2 {
        let projected = self.viewport.project_onto(p);

        Vec2 {
            x: ((projected.x * self.state_size.0 as f64) / self.viewport.w) + (self.state_size.0 as f64 / 2.),
            y: ((projected.y * self.state_size.1 as f64) / self.viewport.h) + (self.state_size.1 as f64 / 2.),
        }
    }

    pub fn append_mesh(&mut self, mesh: &mut Vec<Tri3d>) {
        self.mesh.append(mesh);
    }
}