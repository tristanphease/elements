use bevy::{
    prelude::*, remote::http::RemoteHttpPlugin, remote::RemotePlugin, render::RenderPlugin,
};
use drawable::DrawablePlugin;
use hook::HookPlugin;
use notebook::{
    add_notebook_load, keyboard_animation_control, setup_notebook_animations_once_loaded,
};

use crate::gui::GuiPlugin;

mod camera_controller;
mod drawable;
mod gui;
pub mod hook;
mod notebook;

fn main() {
    let plugin = DefaultPlugins.set(RenderPlugin::default());

    App::new()
        .add_plugins((plugin, DrawablePlugin::default(), HookPlugin))
        // for debugging
        .add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()))
        .add_plugins(GuiPlugin)
        .init_state::<AppState>()
        .add_systems(Startup, add_notebook_load)
        .add_systems(Startup, setup)
        .add_systems(Update, setup_notebook_animations_once_loaded)
        .add_systems(Update, keyboard_animation_control)
        .run();
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, States)]
enum AppState {
    Playing,
    #[default]
    Menu,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 40.0, 0.0).looking_at(Vec3::ZERO, Dir3::NEG_Z), //z up
    ));

    commands.spawn((
        PointLight {
            intensity: 100_000_000_000.0,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 40.0, 0.0),
    ));
}
