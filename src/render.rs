use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::load_objects::{Object, project_vertex};
use crate::uniforms::Uniforms;
use raylib::prelude::*;

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Object],
    uniforms: &Uniforms,
    fov: f32,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    for obj in objects {
        for chunk in obj.indices.chunks_exact(3) {
            // Local vertices of the triangle
            let local_p0 = obj.vertices[chunk[0] as usize];
            let local_p1 = obj.vertices[chunk[1] as usize];
            let local_p2 = obj.vertices[chunk[2] as usize];

            // Transform from local space to world space using the model matrix
            let world_p0 = Vector3::transform(local_p0, uniforms.model_matrix);
            let world_p1 = Vector3::transform(local_p1, uniforms.model_matrix);
            let world_p2 = Vector3::transform(local_p2, uniforms.model_matrix);

            // Project the world space vertices to screen space
            let proj0 = project_vertex(world_p0, uniforms.camera_pos, fov, width, height);
            let proj1 = project_vertex(world_p1, uniforms.camera_pos, fov, width, height);
            let proj2 = project_vertex(world_p2, uniforms.camera_pos, fov, width, height);

            // Draw the edges of the triangle if all vertices are successfully projected
            if let (Some(a), Some(b)) = (proj0, proj1) {
                line(framebuffer, a, b);
            }
            if let (Some(b), Some(c)) = (proj1, proj2) {
                line(framebuffer, b, c);
            }
            if let (Some(c), Some(a)) = (proj2, proj0) {
                line(framebuffer, c, a);
            }
        }
    }
}
