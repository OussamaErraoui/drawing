use crate::shapes::Point; // wla crate::geometrical_shapes::Point 3la 7sab smiya dyal l-fichier dyalek
use raster::{Color, Image};

pub fn render_line(canvas: &mut Image, start: &Point, end: &Point, theme: Color) {
    let mut cur_x = start.x;
    let mut cur_y = start.y;
    let target_x = end.x;
    let target_y = end.y;
    let delta_x = (target_x - cur_x).abs();
    let delta_y = -(target_y - cur_y).abs();

    // 3. Nchoufou l-ittijah (wach ghan-zido l-9eddam wla n-rj3ou l-lor)
    let step_x = if cur_x < target_x { 1 } else { -1 };
    let step_y = if cur_y < target_y { 1 } else { -1 };

    // 4. L-motaghayyir li gha-y3awenna n-s77o l-massar (Error margin)
    let mut error = delta_x + delta_y;

    // 5. Nbdawe n-rsmo f boucle (loop)
    loop {
        // N-lewno l-pixel lhali
        crate::shapes::Displayable::display(canvas, cur_x, cur_y, theme.clone());
        // Ila wslna l-no9ta l-akhira, n7ebso l-boucle
        if cur_x == target_x && cur_y == target_y { 
            break; 
        }
        // N7esbo l-khata2 l-modaa3af (Double error)
        let double_error = 2 * error;

        // Wach n-k7zo f l-mi7war X?
        if double_error >= delta_y {
            error += delta_y;
            cur_x += step_x;
        }
        // Wach n-k7zo f l-mi7war Y?
        if double_error <= delta_x {
            error += delta_x;
            cur_y += step_y;
        }
    }
}