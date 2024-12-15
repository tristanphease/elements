use bevy::{prelude::*, render::{settings::{Backends, WgpuSettings}, RenderPlugin}};
use drawable::drawing_system;
use drawable_material::DrawableMaterial;
use notebook::add_notebook;

mod drawable;
mod drawable_material;
mod notebook;

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
        .add_systems(Startup, add_notebook)
        .add_systems(Startup, setup)
        .add_systems(Update, drawing_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 27.0, 0.0)
            .looking_at(Vec3::ZERO, Dir3::Z), //z up
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

}