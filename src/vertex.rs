use raylib::prelude::Vector3;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vector3,
    pub transformed_position: Vector3,
}

impl Vertex {
    pub fn new(position: Vector3) -> Self {
        Self {
            position,
            transformed_position: position,
        }
    }
}
