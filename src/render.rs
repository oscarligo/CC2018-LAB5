use crate::framebuffer::Framebuffer;
use crate::load_objects::project_vertex;
use crate::primitive::primitive_assembly;
use crate::rasterizer::rasterize;
use crate::shaders::vertex_shader;
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    // 1. Vertex Processing
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| {
            let mut transformed = vertex_shader(vertex, uniforms);
            transformed.screen_position = project_vertex(
                transformed.transformed_position,
                uniforms.camera_pos,
                90.0,
                width,
                height,
            );
            transformed
        })
        .collect();

    // 2. Primitive Assembly
    let triangles = primitive_assembly(&transformed_vertices);

    // 3. Rasterization
    let fragments = rasterize(&triangles);

    // 4. Fragment Processing
    for fragment in fragments {
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            fragment.color,
        );
    }
}
