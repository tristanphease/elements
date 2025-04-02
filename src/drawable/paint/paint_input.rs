use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct PaintInput {
    pub last_input_location: Option<(usize, usize)>,
    pub mouse_down: bool,
}

impl PaintInput {}

impl Default for PaintInput {
    fn default() -> Self {
        Self {
            last_input_location: None,
            mouse_down: false,
        }
    }
}
