use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::load_objects::{Object, project_vertex};
use raylib::prelude::*;

pub fn render(
    framebuffer: &mut Framebuffer,
    obj: &Object,
    camera_pos: Vector3,
    fov: f32,
) {

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    for chunk in obj.indices.chunks_exact(3) {
        let p0 = obj.vertices[chunk[0] as usize];
        let p1 = obj.vertices[chunk[1] as usize];
        let p2 = obj.vertices[chunk[2] as usize];

        let proj0 = project_vertex(p0, camera_pos, fov, width, height);
        let proj1 = project_vertex(p1, camera_pos, fov, width, height);
        let proj2 = project_vertex(p2, camera_pos, fov, width, height);

        // Draw the triangle edges if all vertices are projected successfully
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
