use crate::{math::{tri3d::Tri3d, vec3::Vec3, tri2d::Tri2d, vec2::Vec2, color::{Color, Colors}, mat4::Mat4, transform3d::Transform3d}, state2d::State, drawing, object3d::{Object3d, self}};

/// plane in 3d space that all points are projected onto for rendering
#[derive(Debug, Clone, Copy)]
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

    pub fn projection_mat(self) -> Mat4 {
        Mat4::projection(self.d)
    }
}

pub struct Scene {
    viewport: Viewport,
    state_size: (usize, usize),
    global_transform: Transform3d,
    objects: Vec<Object3d>,
}

impl Scene {
    pub fn new(viewport: Viewport, state_size: (usize, usize), global_transform: Transform3d, objects: Vec<Object3d>) -> Self {
        Scene { viewport, state_size, global_transform, objects}
    }

    pub fn new_empty(state_size: (usize, usize)) -> Self {
        Scene { 
            viewport: Viewport::new(1., 1., 1.), 
            state_size: state_size, 
            global_transform: Transform3d::new_empty(),
            objects: vec![] 
        }
    }

    // pub fn get_state(&self) -> State{
    //     let mut s = State::new_fill(self.state_size, Color::new_color(Colors::BLACK));

    //     // let projected_mesh = self.project_mesh();

    //     // print!("drawing");
    //     for tri in projected_mesh {
    //         // print!("{:?} ", tri);
    //         drawing::tri_wireframe(&mut s, &tri, &Color::new_color(Colors::BLUE));
    //     }

    //     s
    // }

    // fn project_mesh(&self) -> Vec<Tri2d> {
    //     let mut projected_tris = vec![];

    //     for tri in self.mesh.iter() {
    //         projected_tris.push([
    //             self.project_onto_2d(tri[0]),
    //             self.project_onto_2d(tri[1]),
    //             self.project_onto_2d(c),
    //         ])
    //     }

    //     projected_tris
    // }

    pub fn get_state(&self) -> State {
        let projected_mesh = self.projected_mesh();
        let mut s = State::new_fill(self.state_size, Color::gray(0x00));

        for tri in projected_mesh {
            drawing::tri_wireframe(&mut s, &tri, &Color::gray(0xff));
        }
        
        State::new_fill(self.state_size, Color::gray(0xff))
    }

    fn projected_mesh(&self) -> Vec<Tri2d> {
        let mut mesh: Vec<Tri2d> = vec![];

        for (id, object) in self.objects.iter().enumerate() {
            let transform = self.object_mat(id);
            let mut a = object.mesh.iter()
                                    .map(|t| t.apply_transform(transform).into())
                                    .collect::<Vec<Tri2d>>();
            mesh.append(&mut a);
        }

        mesh
    }

    /// gets the transformation 
    fn object_mat(&self, id: usize) -> Mat4 {
        let final_mat: Mat4;

        if let Some(object) = self.objects.get(id) { 
            // gathers all the transformationsacting on an onject and composites them.
            let trans = object.transform.translation_mat() * self.global_transform.translation_mat();
            let rot = object.transform.rotation_mat() * self.global_transform.rotation_mat();
            let scale = object.transform.scale_mat() * self.global_transform.scale_mat();

            let projection = self.viewport.projection_mat();
            let onto2d = self.onto_2d_mat();

            return onto2d * projection * trans * rot * scale
         } else {
            panic!("requested object does not exist");
         }
    }

    fn project_onto_2d(&self, p: &Vec3) -> Vec2 {
        let projected = self.viewport.project_onto(p);

        Vec2 {
            x: ((projected.x * self.state_size.0 as f64) / self.viewport.w) + (self.state_size.0 as f64 / 2.),
            y: ((projected.y * self.state_size.1 as f64) / self.viewport.h) + (self.state_size.1 as f64 / 2.),
        }
    }

    pub fn onto_2d_mat(&self) -> Mat4 {
        //todo do not forget to add half the screen size to adjust the center of screen
        Mat4::onto_2d(self.viewport.w, self.viewport.h, self.state_size.0 as f64, self.state_size.1 as f64)
    }

    // pub fn append_mesh(&mut self, mesh: &mut Vec<Tri3d>) {
    //     self.mesh.append(mesh);
    // }
}