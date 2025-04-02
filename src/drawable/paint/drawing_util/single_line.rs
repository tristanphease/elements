use super::objects::{Axis, Point, Sign};

pub struct PointInfo {
    pub point: Point,
    pub diagonal_move: bool,
}

#[derive(Debug)]
pub struct LineIterator {
    current_main: i32,
    current_sub: i32,
    delta_main: i32,
    delta_sub: i32,
    error: i32,
    end_main: i32,
    axis_main: Axis,
    sign_main: Sign,
    sign_sub: Sign,
}

impl LineIterator {
    pub fn new(start: Point, end: Point) -> Self {
        Self::new_with_error(start, end, 0)
    }

    pub fn new_with_error(start: Point, end: Point, error_offset: i32) -> Self {
        let delta_x = i32::abs(end.0 - start.0);
        let delta_y = i32::abs(end.1 - start.1);
        let axis_main = if delta_x >= delta_y { Axis::X } else { Axis::Y };
        let sign_x = if end.0 >= start.0 {
            Sign::Positive
        } else {
            Sign::Negative
        };
        let sign_y = if end.1 >= start.1 {
            Sign::Positive
        } else {
            Sign::Negative
        };

        let (sign_main, sign_sub) = match axis_main {
            Axis::X => (sign_x, sign_y),
            Axis::Y => (sign_y, sign_x),
        };

        let (delta_main, delta_sub) = match axis_main {
            Axis::X => (delta_x, delta_y),
            Axis::Y => (delta_y, delta_x),
        };

        let (current_main, current_sub, end_main) = match axis_main {
            Axis::X => (start.0, start.1, end.0),
            Axis::Y => (start.1, start.0, end.1),
        };

        let error = delta_sub - delta_main + error_offset;

        Self {
            current_main,
            current_sub,
            delta_main,
            delta_sub,
            error,
            end_main,
            axis_main,
            sign_main,
            sign_sub,
        }
    }

    pub fn get_delta_values(start: Point, end: Point) -> (i32, i32) {
        let delta_x = i32::abs(end.0 - start.0);
        let delta_y = i32::abs(end.1 - start.1);
        let axis_main = if delta_x >= delta_y { Axis::X } else { Axis::Y };

        match axis_main {
            Axis::X => (delta_x, delta_y),
            Axis::Y => (delta_y, delta_x),
        }
    }

    pub fn get_sub_sign(start: Point, end: Point) -> Sign {
        let delta_x = i32::abs(end.0 - start.0);
        let delta_y = i32::abs(end.1 - start.1);
        let axis_main = if delta_x >= delta_y { Axis::X } else { Axis::Y };
        let sign_x = if end.0 >= start.0 {
            Sign::Positive
        } else {
            Sign::Negative
        };
        let sign_y = if end.1 >= start.1 {
            Sign::Positive
        } else {
            Sign::Negative
        };

        match axis_main {
            Axis::X => sign_y,
            Axis::Y => sign_x,
        }
    }

    pub fn get_sign_main(&self) -> Sign {
        self.sign_main
    }

    pub fn get_sign_sub(&self) -> Sign {
        self.sign_sub
    }

    pub fn get_axis_main(&self) -> Axis {
        self.axis_main
    }

    fn completed(&self) -> bool {
        match self.sign_main {
            Sign::Positive => self.current_main > self.end_main,
            Sign::Negative => self.current_main < self.end_main,
        }
    }

    fn get_current_point(&self) -> Point {
        match self.axis_main {
            Axis::X => Point(self.current_main, self.current_sub),
            Axis::Y => Point(self.current_sub, self.current_main),
        }
    }
}

impl Iterator for LineIterator {
    type Item = PointInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed() {
            return None;
        }

        let current_point = self.get_current_point();

        let mut diagonal_move = false;

        if self.error >= 0 {
            self.current_sub += i32::from(self.sign_sub);
            self.error -= self.delta_main;
            diagonal_move = true;
        }
        self.error += self.delta_sub;

        self.current_main += i32::from(self.sign_main);

        Some(PointInfo {
            point: current_point,
            diagonal_move,
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::objects::Point;
    use super::LineIterator;

    #[test]
    fn test_angled_line() {
        let line_iterator = LineIterator::new(Point(3, 1), Point(6, 4));
        let line_result: Vec<_> = line_iterator.map(|point_info| point_info.point).collect();

        assert_eq!(
            line_result,
            vec![Point(3, 1), Point(4, 2), Point(5, 3), Point(6, 4)]
        );
    }

    #[test]
    fn test_vertical_line() {
        let line_iterator = LineIterator::new(Point(3, 0), Point(3, 4));
        let line_results: Vec<_> = line_iterator.map(|point_info| point_info.point).collect();

        assert_eq!(
            line_results,
            vec![
                Point(3, 0),
                Point(3, 1),
                Point(3, 2),
                Point(3, 3),
                Point(3, 4)
            ]
        );
    }

    #[test]
    fn test_horizontal_line() {
        let line_iterator = LineIterator::new(Point(4, 2), Point(1, 2));
        let line_results: Vec<_> = line_iterator.map(|point_info| point_info.point).collect();

        assert_eq!(
            line_results,
            vec![Point(4, 2), Point(3, 2), Point(2, 2), Point(1, 2)]
        );
    }
}
