use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct PaintInput {
    pub last_input_location: Option<(usize, usize)>,
    pub mouse_down: bool,
}
