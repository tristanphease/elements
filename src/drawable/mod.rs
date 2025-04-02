pub mod drawable;
pub mod drawable_builder;
pub mod drawable_material;
mod paint;

use bevy::app::Plugin;
use bevy::app::Update;
use bevy::pbr::MaterialPlugin;
use drawable_builder::add_drawable_system;
use paint::PaintPlugin;

//re-export
pub use crate::drawable::drawable::*;
pub use crate::drawable::drawable_material::*;

#[derive(Debug, Default)]
pub struct DrawablePlugin {}

impl Plugin for DrawablePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MaterialPlugin::<DrawableMaterial>::default());
        app.add_plugins(PaintPlugin::default());

        app.add_systems(Update, add_drawable_system);
        app.add_systems(Update, drawing_system);
    }
}
