#![expect(dead_code)]

// resource for drawing thick lines

use super::{
    objects::{Point, Sign},
    single_line::LineIterator,
};

#[derive(Debug)]
enum SpecialLine {
    Yes(Point),
    No,
}

#[derive(Debug)]
pub struct ThickLine {
    start: Point,
    end: Point,

    main_line_iterator: LineIterator,
    sub_line_iterator: Option<LineIterator>,

    // extra info
    special_line: SpecialLine,
    perpendicular_error: i32,
    error_sign: Sign,
    sub_delta_main: i32,
    sub_delta_sub: i32,
}

impl ThickLine {
    pub fn new(start: Point, end: Point, thickness: f32) -> Self {
        let (main_point1, main_point2) = Point::get_perpendicular_points(&start, &end, thickness);

        let main_line_iterator = LineIterator::new(main_point1, main_point2);
        let sub_line_iterator = None;

        let (sub_delta_main, sub_delta_sub) = LineIterator::get_delta_values(start, end);

        let error_sign =
            !if main_line_iterator.get_sign_main() == LineIterator::get_sub_sign(start, end) {
                Sign::Positive
            } else {
                Sign::Negative
            };

        Self {
            start,
            end,
            main_line_iterator,
            sub_line_iterator,
            special_line: SpecialLine::No,
            perpendicular_error: 0,
            error_sign,
            sub_delta_main,
            sub_delta_sub,
        }
    }
}

impl Iterator for ThickLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut sub_line_iterator) = self.sub_line_iterator {
            if let Some(new_pixel_info) = sub_line_iterator.next() {
                return Some(new_pixel_info.point);
            }
        }

        if let SpecialLine::Yes(next_start_point) = self.special_line {
            let diff_point = self.end - self.start;
            let new_end_point = next_start_point + diff_point;

            self.perpendicular_error += self.sub_delta_sub - self.sub_delta_main;

            let new_sub_line_iterator = LineIterator::new_with_error(
                next_start_point,
                new_end_point,
                i32::from(self.error_sign) * self.perpendicular_error,
            );
            self.sub_line_iterator = Some(new_sub_line_iterator);

            self.special_line = SpecialLine::No;
            return self.next();
        }

        if let Some(next_start_point_info) = self.main_line_iterator.next() {
            let next_start_point = next_start_point_info.point;
            let diff_point = self.end - self.start;
            let new_end_point = next_start_point + diff_point;
            let new_sub_line_iterator = LineIterator::new_with_error(
                next_start_point,
                new_end_point,
                i32::from(self.error_sign) * self.perpendicular_error,
            );

            if next_start_point_info.diagonal_move {
                if self.perpendicular_error >= -(self.sub_delta_sub - self.sub_delta_main) {
                    let sub_sub_axis = new_sub_line_iterator.get_axis_main().other();
                    let sub_sign = new_sub_line_iterator.get_sign_sub();
                    let special_start_point =
                        next_start_point.add_axis(sub_sub_axis, i32::from(sub_sign));
                    self.special_line = SpecialLine::Yes(special_start_point);
                } else {
                    self.perpendicular_error += self.sub_delta_sub;
                }
            }

            self.sub_line_iterator = Some(new_sub_line_iterator);
            return self.next();
        }

        None
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use image::RgbaImage;

    use super::super::objects::Point;
    use super::ThickLine;

    #[test]
    fn basic_horizontal_thick_line() {
        let thick_line_iterator = ThickLine::new(Point(10, 10), Point(50, 10), 5.0);
        let line_result: HashSet<_> = thick_line_iterator.collect();

        for x in 10..=50 {
            for y in 5..=15 {
                let point = Point(x, y);
                if !line_result.contains(&point) {
                    println!("{point:?}");
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn angled_line() {
        let thick_line_iterator = ThickLine::new(Point(10, 10), Point(20, 5), 3.0);
        let line_result: HashSet<_> = thick_line_iterator.collect();

        points_to_image(&line_result, 30, 1);

        // println!("{:?}", line_result);
    }

    fn points_to_image(points: &HashSet<Point>, size: usize, num: i32) {
        let mut data = vec![255; size * size * 4];
        for point in points.iter() {
            data[4 * (point.1 as usize * size + point.0 as usize)] = 0;
            data[4 * (point.1 as usize * size + point.0 as usize) + 1] = 0;
            data[4 * (point.1 as usize * size + point.0 as usize) + 2] = 0;
            data[4 * (point.1 as usize * size + point.0 as usize) + 3] = 255;
        }
        let image = RgbaImage::from_vec(size as u32, size as u32, data).unwrap();

        let output = image.save(format!("./output{}.png", num));
        println!("{:?}", output);
    }
}
