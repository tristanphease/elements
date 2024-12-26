use bevy::{prelude::*, render::{settings::{Backends, WgpuSettings}, RenderPlugin}};
use drawable::drawing_system;
use drawable::DrawableMaterial;
use notebook::{add_notebook_load, keyboard_animation_control, setup_notebook_animations_once_loaded};

mod drawable;

mod notebook;
mod camera_controller;
fn main() {
    let plugin = DefaultPlugins
        .set(RenderPlugin {
            render_creation: WgpuSettings {
                // https://github.com/gfx-rs/wgpu/issues/4247
                backends: Some(Backends::VULKAN),
                ..default()
            }.into(),
            ..default()
        });

    App::new()
        .add_plugins((plugin, MaterialPlugin::<DrawableMaterial>::default()))
        .add_systems(Startup, add_notebook_load)
        .add_systems(Startup, setup)
        .add_systems(Update, drawing_system)
        .add_systems(Update, setup_notebook_animations_once_loaded)
        .add_systems(Update, keyboard_animation_control)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 50.0, 0.0)
            .looking_at(Vec3::ZERO, Dir3::NEG_Z), //z up
    ));

    // commands.spawn((
    //     PointLight {
    //         shadows_enabled: true,
    //         intensity: 10_000_000.0,
    //         range: 100.0,
    //         shadow_depth_bias: 0.2,
    //         ..default()
    //     },
    //     Transform::from_xyz(0.0, 16.0, 0.0),
    // ));

    commands.spawn((
        DirectionalLight {
            illuminance: 100_000_000_000.0,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 50.0, 0.0),
    ));

}