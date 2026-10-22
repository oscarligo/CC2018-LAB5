use raylib::prelude::{Color, Vector2};

#[derive(Clone, Debug)]
pub struct Fragment {
    pub position: Vector2,
    pub color: Color,
    pub depth: f32,
}
