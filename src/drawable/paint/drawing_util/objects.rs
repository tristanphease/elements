use std::ops::{Add, Not, Sub};

use bevy::math::Vec2;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn get_perpendicular_points(&self, point2: &Point, distance: f32) -> (Point, Point) {
        let point1 = Vec2::new(self.0 as f32, self.1 as f32);
        let point2 = Vec2::new(point2.0 as f32, point2.1 as f32);

        let point_diff = (point2 - point1).normalize();
        let orthogonal_point = Vec2::new(-point_diff.y, point_diff.x);

        let new_point1 = point1 + orthogonal_point * distance;
        let new_point2 = point1 + orthogonal_point * -distance;

        fn convert_float(val: f32) -> i32 {
            val.round() as i32
        }

        fn convert_point(val: Vec2) -> Point {
            Point(convert_float(val.x), convert_float(val.y))
        }

        (convert_point(new_point1), convert_point(new_point2))
    }

    pub fn add_axis(&self, axis: Axis, amount: i32) -> Point {
        let mut point = Point(self.0, self.1);
        match axis {
            Axis::X => point.0 += amount,
            Axis::Y => point.1 += amount,
        };
        point
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn other(&self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Positive,
    Negative,
}

impl From<Sign> for i32 {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Positive => 1,
            Sign::Negative => -1,
        }
    }
}

impl Not for Sign {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}
