use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

// bresenham's line algorithm
pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
) {
    let mut x0 = start.x.round() as i32;
    let mut y0 = start.y.round() as i32;        
    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;  

    let dx = (x1 - x0).abs(); 
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };  

    let mut err = dx - dy; 

    loop {
        // Validación de márgenes de pantalla para evitar panics
        if x0 >= 0 && x0 < framebuffer.width as i32 && y0 >= 0 && y0 < framebuffer.height as i32 {
            framebuffer.set_pixel(x0 as u32, y0 as u32);
        }

        if x0 == x1 && y0 == y1 { break; }

        let e2 = 2 * err;
        if e2 >= -dy { 
            err -= dy; 
            x0 += sx; 
        } 
        if e2 <= dx { 
            err += dx; 
            y0 += sy; 
        } 
    }
}
