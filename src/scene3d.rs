use crate::{math::{tri3d::Tri3d, vec3::Vec3, tri2d::Tri2d, vec2::Vec2, color::{Color, Colors}, mat4::Mat4, transform3d::Transform3d, vec4::Vec4}, state2d::State, drawing, object3d::{Object3d, self}};

/// plane in 3d space that all points are projected onto for rendering
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    w: f64,
    h: f64,
    d: f64,
}

impl Viewport {
    pub fn new(w: f64, h:f64, d:f64) -> Self{
        Viewport { w, h, d }
    }

    pub fn _project_onto(&self, p: &Vec3) -> Vec3 {
        // deprecated, functionality is now handled with a matrix
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

// a plane is defined by the equation:
// Ax + By + Cz + D = 0
// or 
// N⋅P + D = 0
// where 
// N = (A, B, C)
// P = (x, y, z)
pub struct ClippingPlane {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl ClippingPlane {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        ClippingPlane { a, b, c, d }
    }

    pub fn new_normal(n: Vec3, d:f64) -> Self {
        ClippingPlane::new(
            n.x, 
            n.y, 
            n.z, 
            d
        )
    }

    pub fn clip_point(&self, p: Vec4) -> bool {
        ( self.a * p.x ) + ( self.b * p.y ) + ( self.c * p.z ) + self.d >= 0.
    }
}

pub struct Scene {
    viewport: Viewport,
    state_size: (usize, usize),
    pub global_transform: Transform3d,
    clipping_planes: Vec<ClippingPlane>,
    pub objects: Vec<Object3d>,
}

impl Scene {
    pub fn new(viewport: Viewport, state_size: (usize, usize), global_transform: Transform3d, clipping_planes: Vec<ClippingPlane>, objects: Vec<Object3d>) -> Self {
        Scene { viewport, state_size, global_transform, clipping_planes, objects }
    }

    pub fn new_empty(state_size: (usize, usize)) -> Self {
        let viewport = Viewport::new(1., 1., 1.);
        
        // create clipping planes
        const OORT: f64 = 0.70710678; // one over root two
        let clipping_planes = vec![
            ClippingPlane::new(0., 0., 1., viewport.d), // near
            ClippingPlane::new(OORT, 0., OORT, 0.),     // left
            ClippingPlane::new(-OORT, 0., OORT, 0.),    // right
            ClippingPlane::new(0., OORT, OORT, 0.),     // bottom
            ClippingPlane::new(0., -OORT, OORT, 0.),    // top
        ];

        Scene { 
            viewport, 
            state_size, 
            global_transform: Transform3d::new_empty(),
            clipping_planes,
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

        assert_ne!(projected_mesh.len(), 0);

        for tri in projected_mesh {
            drawing::tri_wireframe(&mut s, &tri, &Color::new_color(Colors::GREEN));
            // drawing::tri_filled(&mut s, &tri, &tri.color);
        }
        s
    }

    fn projected_mesh(&self) -> Vec<Tri2d> {
        let mut mesh: Vec<Tri2d> = vec![];

        for (id, object) in self.objects.iter().enumerate() {
            let transform = self.object_mat(id);
            for tri in object.mesh.iter() {
                let tri_new = tri.apply_transform(transform);
                let a: Vec2 = Vec2::new(self.state_size.0 as f64 / 2., self.state_size.0 as f64 / 2.);
                
                let mut points: [Vec2; 3] = [
                    tri_new[0].into(),
                    tri_new[1].into(),
                    tri_new[2].into(),
                ];

                points = [
                    points[0] + a,
                    points[1] + a,
                    points[2] + a,
                ];
                
                mesh.push(Tri2d::new(points, tri.color));
            }
        }
        
        mesh
    }

    /// gets the transformation matrix of a given object
    fn object_mat(&self, id: usize) -> Mat4 {
        if let Some(object) = self.objects.get(id) { 
            // gathers all the transformations acting on an onject and composites them.
            let trans = object.transform.translation_mat() * self.global_transform.translation_mat();
            let rot = object.transform.rotation_mat() * self.global_transform.rotation_mat();
            let scale = object.transform.scale_mat() * self.global_transform.scale_mat();

            let projection = self.viewport.projection_mat();
            let onto2d = self.onto_2d_mat();
            let fix_to_center = self.fix_to_center_mat();

            // return fix_to_center * (onto2d * projection * trans * rot * scale)
            return onto2d * projection * trans * rot * scale
         } else {
            panic!("requested object does not exist");
         }
    }

    fn _project_onto_2d(&self, p: &Vec3) -> Vec2 {
        // deprecated, functionality handled through matrices
        let projected = self.viewport._project_onto(p);

        Vec2 {
            x: ((projected.x * self.state_size.0 as f64) / self.viewport.w) + (self.state_size.0 as f64 / 2.),
            y: ((projected.y * self.state_size.1 as f64) / self.viewport.h) + (self.state_size.1 as f64 / 2.),
        }
    }

    pub fn onto_2d_mat(&self) -> Mat4 {
        //todo do not forget to add half the screen size to adjust the center of screen
        Mat4::onto_2d(self.viewport.w, self.viewport.h, self.state_size.0 as f64, self.state_size.1 as f64)
    }

    pub fn fix_to_center_mat(&self) -> Mat4 {
        Mat4::fix_to_center(self.state_size.0 as f64, self.state_size.1 as f64)
    }
 
    pub fn add_object(&mut self, object: Object3d) {
        println!("added object to scene!");
        self.objects.push(object);
    }
}