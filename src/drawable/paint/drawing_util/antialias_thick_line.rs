use super::objects::Point;

/// Draws an antialiased line using gupta-sproull method
pub fn draw_antialiased_thick_line<F>(start: Point, end: Point, width: f32, mut draw_fn: F)
where
    F: FnMut(Point, f32),
{
    // handle all octants
    let mut x0 = start.0;
    let mut y0 = start.1;
    let mut x1 = end.0;
    let mut y1 = end.1;

    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    // swap coordinates if the line is steep (dy > dx) so we're always iterating on the x-axis
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    // ensure x0 <= x1
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let delta_x = (x1 - x0) as f32;
    let delta_y = (y1 - y0) as f32;
    let length_sq = delta_x.powi(2) + delta_y.powi(2);

    let inv_length = if length_sq == 0.0 {
        1.0
    } else {
        1.0 / length_sq.sqrt()
    };

    let a = -delta_y;
    let b = delta_x;

    let half_width = width / 2.0;

    let calc_intensity = |d: f32, half_w: f32| -> f32 {
        let distance_from_edge = half_w - d.abs();
        (distance_from_edge + 0.5).clamp(0.0, 1.0)
    };

    let mut y_center_f = y0 as f32;
    let gradient = delta_y / delta_x;

    for x in x0..=x1 {
        let y_center_i = y_center_f.round() as i32;

        let perp_range = (half_width + 1.5).ceil() as i32;

        for y_offset in -perp_range..=perp_range {
            let y = y_center_i + y_offset;

            let x_f = x as f32;
            let y_f = y as f32;
            let x0_f = x0 as f32;
            let y0_f = y0 as f32;

            let numerator = (a * (x_f - x0_f) + b * (y_f - y0_f)).abs();

            let d = numerator * inv_length;

            if d < half_width + 1.0 {
                let alpha = calc_intensity(d, half_width);

                let point = if steep { Point(y, x) } else { Point(x, y) };
                draw_fn(point, alpha);
            }
        }

        y_center_f += gradient;
    }
}
