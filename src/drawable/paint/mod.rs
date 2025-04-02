use bevy::prelude::*;
use paint_input::PaintInput;
// handles painting on a texture

mod drawing_util;
pub mod paint;
pub mod paint_input;

#[derive(Debug, Default)]
pub struct PaintPlugin {}

impl Plugin for PaintPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PaintInput>();
    }
}
