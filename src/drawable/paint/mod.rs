use bevy::prelude::*;
use paint_input::PaintInput;
// handles painting on a texture

use bevy::color::Alpha;
use bevy::{color::Color, image::Image, math::Vec2};

use drawing_util::antialias_thick_line::draw_antialiased_thick_line;
use drawing_util::{objects::Point, thick_line::ThickLine};

mod drawing_util;
pub mod paint_input;

#[derive(Debug, Default)]
pub struct PaintPlugin {}

impl Plugin for PaintPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PaintInput>();
    }
}

pub struct PaintSettings {
    radius: f32,
    colour: Color,
}

impl Default for PaintSettings {
    fn default() -> Self {
        Self {
            radius: 0.05,
            colour: Color::BLACK,
        }
    }
}

pub trait PaintImage {
    fn draw_spot(&mut self, x: usize, y: usize, paint_settings: &PaintSettings, plane_scale: Vec2);

    #[expect(dead_code)]
    fn draw_thick_line(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        paint_settings: &PaintSettings,
        plane_scale: Vec2,
    );

    fn draw_thick_line_antialias(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        paint_settings: &PaintSettings,
        plane_scale: Vec2,
    );
}

impl PaintImage for Image {
    // TODO: make this more efficient
    fn draw_spot(&mut self, x: usize, y: usize, paint_settings: &PaintSettings, plane_scale: Vec2) {
        // radius needs to not care about resolution, make it
        let scale_factor = self.width() as f32 / plane_scale.x;
        let radius_f = paint_settings.radius * scale_factor;

        let radius = radius_f as usize;

        let min_x = usize::checked_sub(x, radius).unwrap_or(0);
        let min_y = usize::checked_sub(y, radius).unwrap_or(0);

        let max_x = usize::min(x + radius, self.width() as usize - 1);
        let max_y = usize::min(y + radius, self.height() as usize - 1);

        for x_val in min_x..=max_x {
            for y_val in min_y..=max_y {
                let distance = f32::hypot(y_val as f32 - y as f32, x_val as f32 - x as f32);
                if distance <= radius_f {
                    let _ = self.set_color_at(x_val as u32, y_val as u32, paint_settings.colour);
                }
            }
        }
    }

    /// draw a thick line from point 1 to point 2
    /// adapted from http://kt8216.unixcab.org/murphy/index.html
    /// https://www.research-collection.ethz.ch/handle/20.500.11850/68976
    fn draw_thick_line(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        paint_settings: &PaintSettings,
        plane_scale: Vec2,
    ) {
        let start_point = Point(i32::try_from(x1).unwrap(), i32::try_from(y1).unwrap());
        let end_point = Point(i32::try_from(x2).unwrap(), i32::try_from(y2).unwrap());

        let scale_factor = self.width() as f32 / plane_scale.x;
        let radius_f = paint_settings.radius * scale_factor;
        let thick_line = ThickLine::new(start_point, end_point, radius_f);

        for point in thick_line {
            if point.0 >= 0 && point.1 >= 0 {
                let _ = self.set_color_at(point.0 as u32, point.1 as u32, paint_settings.colour);
            }
        }
    }

    fn draw_thick_line_antialias(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        paint_settings: &PaintSettings,
        plane_scale: Vec2,
    ) {
        let start_point = Point(i32::try_from(x1).unwrap(), i32::try_from(y1).unwrap());
        let end_point = Point(i32::try_from(x2).unwrap(), i32::try_from(y2).unwrap());

        let scale_factor = self.width() as f32 / plane_scale.x;
        let radius_f = paint_settings.radius * scale_factor;
        draw_antialiased_thick_line(start_point, end_point, radius_f * 2.0, |point, amount| {
            let colour = paint_settings.colour.with_alpha(amount);
            let _ = self.set_color_at(point.0 as u32, point.1 as u32, colour);
        });
    }
}
