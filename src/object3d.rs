use std::{fs::File, io::BufReader};
use std::io::BufRead;

use crate::math::{tri3d::Tri3d, transform3d::{Transform3d, self}, mat4::Mat4, vec4::Vec4, color::{Color, Colors}};

#[derive(Debug, Clone)]
pub struct Object3d {
    pub key: u32,
    pub transform: Transform3d,
    pub mesh: Vec<Tri3d>,
}

impl Object3d {
    pub fn cube() -> Self {
        let vs: [Vec4; 8] = [
            Vec4::new( 1.,  1.,  1.,  1.),
            Vec4::new(-1.,  1.,  1.,  1.),
            Vec4::new(-1., -1.,  1.,  1.),
            Vec4::new( 1., -1.,  1.,  1.),
            Vec4::new( 1.,  1., -1.,  1.),
            Vec4::new(-1.,  1., -1.,  1.),
            Vec4::new(-1., -1., -1.,  1.),
            Vec4::new( 1., -1., -1.,  1.),
        ];

        let mesh: Vec<Tri3d> = vec![
            Tri3d::new([vs[0], vs[1], vs[2]], Color::new_color(Colors::RED)), 
            Tri3d::new([vs[0], vs[2], vs[3]], Color::new_color(Colors::RED)),
            Tri3d::new([vs[4], vs[0], vs[3]], Color::new_color(Colors::GREEN)),
            Tri3d::new([vs[4], vs[3], vs[7]], Color::new_color(Colors::GREEN)),
            Tri3d::new([vs[5], vs[4], vs[7]], Color::new_color(Colors::BLUE)),
            Tri3d::new([vs[5], vs[7], vs[6]], Color::new_color(Colors::BLUE)),
            Tri3d::new([vs[1], vs[5], vs[6]], Color::new_color(Colors::YELLOW)),
            Tri3d::new([vs[1], vs[6], vs[2]], Color::new_color(Colors::YELLOW)),
            Tri3d::new([vs[4], vs[5], vs[1]], Color::new_color(Colors::MAGENTA)),
            Tri3d::new([vs[4], vs[1], vs[0]], Color::new_color(Colors::MAGENTA)),
            Tri3d::new([vs[2], vs[6], vs[7]], Color::new_color(Colors::CYAN)),
            Tri3d::new([vs[2], vs[7], vs[3]], Color::new_color(Colors::CYAN)),
        ];

        Object3d { key: 0, transform: Transform3d::new_empty(), mesh }
    }

    pub fn plane() -> Self {
        let vs: [Vec4; 4] = [
            Vec4::new( 1.,  1.,  0.,  1.),
            Vec4::new(-1.,  1.,  0.,  1.),
            Vec4::new(-1., -1.,  0.,  1.),
            Vec4::new( 1., -1.,  0.,  1.),
        ];

        let mesh: Vec<Tri3d> = vec![
            Tri3d::new([vs[0], vs[2], vs[1]], Color::new_color(Colors::RED)), 
            Tri3d::new([vs[0], vs[2], vs[3]], Color::new_color(Colors::BLUE))
        ];
        
        Object3d { key: 1, transform: Transform3d::new_empty(), mesh }
    }

    pub fn model_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // println!("{}", std::env::current_dir()?.display());
        let file = File::open(path).expect("could not read file");
        let reader = BufReader::new(file);

        let mut vertices: Vec<Vec4> = vec![];
        let mut mesh: Vec<Tri3d> = vec![];

        for line in reader.lines() { // loop through all lines in the fie
            let slice: &str = &line?[..];
            // print!("{} ", slice.chars().next().unwrap());

            if slice.starts_with("v ") { // if this is the definition of a vertex
                let split: Vec<&str> = slice.split(" ").collect();
                // println!("{:?}", split);

                vertices.push(
                    Vec4::new(
                        split[1].parse::<f64>().unwrap(), 
                        split[2].parse::<f64>().unwrap(), 
                        split[3].parse::<f64>().unwrap(), 
                        1.
                    )
                )
            }

            if slice.starts_with("f ") { // if this is a face

                // f v1/vn1/vt1 v2/vn2/vt2 v3/vn3/vt3
                let split = slice.trim_matches('f')
                                                    .trim()
                                                    .split(" ")
                                                    .collect::<Vec<&str>>();
                // ["v1/vn1/vt1", "v2/vn2/vt2", "v3/vn3/vt3"]

                let mut tri_vertices = [Vec4::zero(); 3];

                for (i, vertex_data) in split.iter().enumerate() {
                    let v_id = vertex_data.split("/")
                                                    .next() // only need the first bit of data on each vertex
                                                    .unwrap()
                                                    .parse::<usize>()
                                                    .unwrap(); 

                    tri_vertices[i] = vertices[v_id - 1]; //  -1 becauee v_ids start at 1
                }

                mesh.push( Tri3d::new(tri_vertices, Color::gray(255)) );
            }   
        }

        Ok(
            Object3d { key: 1, transform: Transform3d::new_empty(), mesh }
        )
    }
}