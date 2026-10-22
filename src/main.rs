mod framebuffer;
mod fragment;
mod line;
mod load_objects;
mod primitive;
mod rasterizer;
mod render;
mod shaders;
mod triangle;
mod uniforms;
mod vertex;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use render::render;
use std::thread;
use std::time::Duration;
use uniforms::Uniforms;

fn main() {
    const WINDOW_WIDTH: i32 = 600;
    const WINDOW_HEIGHT: i32 = 650;

    let (mut window, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .build();

    // Load the 3D object from an .obj file
    let vertex_array =
        load_objects::load_obj("assets/object.obj").expect("Failed to load .obj file");
    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, Color::BLACK);

    // moves the camera position to view the object from a different angle
    let camera_pos = Vector3::new(10.0, 2.0, -2.0);
    let mut uniforms = Uniforms {
        model_matrix: Matrix::identity(),
        camera_pos,
    };

    while !window.window_should_close() {
        framebuffer.clear();

        // Handle camera movement based on user input
        if window.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
            // Move camera forward
            uniforms.camera_pos += Vector3::new(0.0, 0.1, 0.0);
        }
        if window.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
            // Move camera backward
            uniforms.camera_pos -= Vector3::new(0.0, 0.1, 0.0);
        }
        if window.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            // Move camera left
            uniforms.camera_pos -= Vector3::new(0.1, 0.0, 0.0);
        }
        if window.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
            // Move camera right
            uniforms.camera_pos += Vector3::new(0.1, 0.0, 0.0);
        }
        // Render the 3D object to the framebuffer
        render(&mut framebuffer, &uniforms, &vertex_array);

        framebuffer.swap_buffers(&mut window, &thread);
        thread::sleep(Duration::from_millis(16));
    }
}
