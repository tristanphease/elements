pub mod drawable;
pub mod drawable_builder;
pub mod drawable_material;
// for image related things to do with drawing
mod drawable_image;
mod paint;

use bevy::app::Plugin;
use bevy::app::Update;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::pbr::MaterialPlugin;
use bevy::state::condition::in_state;
use drawable_builder::add_drawable_system;
use paint::PaintPlugin;

//re-export
pub use crate::drawable::drawable::*;
use crate::drawable::drawable_image::clear_drawable_image;
use crate::drawable::drawable_image::save_drawable_image;
pub use crate::drawable::drawable_material::*;
use crate::AppState;
pub(crate) use drawable_image::ClearDrawableImage;
pub(crate) use drawable_image::SaveDrawableImage;

#[derive(Debug, Default)]
pub struct DrawablePlugin {}

impl Plugin for DrawablePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MaterialPlugin::<DrawableMaterial>::default());
        app.add_plugins(PaintPlugin::default());

        app.add_systems(Update, add_drawable_system);
        app.add_systems(Update, drawing_system.run_if(in_state(AppState::Playing)));

        // debug stuff
        app.add_message::<SaveDrawableImage>();
        app.add_systems(Update, save_drawable_image);
        app.add_message::<ClearDrawableImage>();
        app.add_systems(Update, clear_drawable_image);
    }
}
