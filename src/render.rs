use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::load_objects::project_vertex;
use crate::shaders::vertex_shader;
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    // 1. Vertex Processing
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // 2. Primitive Assembly
    for triangle in transformed_vertices.chunks_exact(3) {
        let (a, b, c) = (&triangle[0], &triangle[1], &triangle[2]);
        let projected = [a, b, c].map(|vertex| {
            project_vertex(
                vertex.transformed_position,
                uniforms.camera_pos,
                90.0,
                width,
                height,
            )
        });
        let [Some(a), Some(b), Some(c)] = projected else {
            continue;
        };

        // 3. Rasterization
        let fragments = line(a, b, framebuffer.width, framebuffer.height)
            .into_iter()
            .chain(line(b, c, framebuffer.width, framebuffer.height))
            .chain(line(c, a, framebuffer.width, framebuffer.height));

        // 4. Fragment Processing
        for (x, y) in fragments {
            framebuffer.set_pixel(x, y);
        }
    }
}
