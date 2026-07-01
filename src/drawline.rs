use crate::shapes::Point;
use raster::{Color, Image};

pub fn draw_line_same_color(image: &mut Image, p1: &Point, p2: &Point, color: Color) {
    print!("mmm");
    // DDA (Digital Differential Analyzer) line drawing
    let x1 = p1.x as f32;
    let y1 = p1.y as f32;
    let x2 = p2.x as f32;
    let y2 = p2.y as f32;

    let dx = x2 - x1;
    let dy = y2 - y1;

    let steps_f = dx.abs().max(dy.abs());

    let x_inc = dx / steps_f as f32;
    let y_inc = dy / steps_f as f32;

    let mut x = x1;
    let mut y = y1;

    for _ in 0..=steps_f as i32 {
        crate::shapes::Displayable::display(image, x as i32, y as i32, color.clone());
        x += x_inc;
        y += y_inc;
    }
}
//
// pub fn draw_line_same_color(image: &mut Image, p1: Point, p2: Point, color: Color) {
//     let mut x1 = p1.x;
//     let mut y1 = p1.y;
//     let x2 = p2.x;
//     let y2 = p2.y;

//     let dx = (x2 - x1).abs();
//     let dy = -(y2 - y1).abs();

//     let sx = if x1 < x2 { 1 } else { -1 };
//     let sy = if y1 < y2 { 1 } else { -1 };

//     let mut err = dx + dy;

//     loop {
//         crate::geometrical_shapes::Displayable::display(image, x1, y1, color.clone());

//         if x1 == x2 && y1 == y2 { break; }

//         let e2 = 2 * err;

//         if e2 >= dy {
//             err += dy;
//             x1 += sx;
//         }

//         if e2 <= dx {
//             err += dx;
//             y1 += sy;
//         }
//     }
// }
