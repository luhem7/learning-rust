use std::fmt;
use random_color::{
    RandomColor,
    Luminosity,
};

struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex{
    fn normalize_color_val(input: u32) -> f32{
        return input as f32 / 255.0;
    }

    fn new(x:f32, y:f32, z:f32) -> Self{
        let color = RandomColor::new()
            .luminosity(Luminosity::Light)
            .to_rgb_array();
        
        return Vertex{
            position : [x, y, z],
            color :  [Vertex::normalize_color_val(color[0]),
                Vertex::normalize_color_val(color[1]),
                Vertex::normalize_color_val(color[2]),]
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos ({}, {}, {}) ; Color ({}, {}, {})", self.position[0], self.position[1], self.position[2],
            self.color[0], self.color[1], self.color[2],)
    }
}



fn calc_vertices_indices(num_vertices : u64) -> Option<(Vec<Vertex>, Vec<u16>)>{
    let radius = 2.0;
    println!("Creating shape with radius {}", radius);

    const PI:f32 = std::f32::consts::PI;
    let angle_step:f32 = 2.0*PI / num_vertices as f32;

    let mut vertices : Vec<Vertex> = Vec::new();
    for curr_vertex in 0..num_vertices{
        println!("Curr Vertex {}", curr_vertex);

        let curr_angle = curr_vertex as f32*angle_step;
        vertices.push(Vertex::new(radius*curr_angle.cos(), radius*curr_angle.sin(), 0.0));
        
        println!("Co-ords {}", vertices.last().unwrap());
    }
}

fn main() {
    calc_vertices_indices(3);
}
