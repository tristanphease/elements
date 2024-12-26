use std::time::Duration;

use bevy::{color, prelude::*};

use crate::drawable::{drawable_builder::BuildDrawable, Drawable, DrawableMaterial};

const NOTEBOOK_PATH: &str = "models/notebook.glb";

pub fn add_notebook(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut drawable_materials: ResMut<Assets<DrawableMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cuboid = Cuboid::from_size(Vec3::new(10.0, 5.0, 20.0));
    let mesh_handle: Handle<Mesh> = meshes.add(cuboid);

    let material = standard_materials.add(Color::from(color::palettes::basic::GRAY));

    let mut notebook_entity = commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Drawable,
    ));

    notebook_entity.add_drawable(
        &mesh_handle,
        &mut meshes,
        &mut drawable_materials,
        &asset_server
    );

}

pub fn add_notebook_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset(NOTEBOOK_PATH)
    )));

    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(NOTEBOOK_PATH)),
    ]);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(NotebookAnimations {
        animations: node_indices,
        graph: graph_handle
    });
    // println!("{graph:?}, {node_indices:?}");
}

pub fn setup_notebook_animations_once_loaded(
    mut commands: Commands,
    animations: Res<NotebookAnimations>,
    mut anim_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut anim_players {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO);
            // .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
    
}

#[derive(Resource)]
pub struct NotebookAnimations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

pub fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    // animations: Res<Animations>,
    // mut current_animation: Local<usize>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
            continue;
        };

        if keyboard_input.just_pressed(KeyCode::Space) {
            let playing_animation_option = player.animation_mut(playing_animation_index);
            
            if let Some(playing_animation) = playing_animation_option {
                if playing_animation.is_finished() {
                    let current_time = playing_animation.seek_time();
                    playing_animation.replay();
                    playing_animation.set_seek_time(current_time);
                }
                playing_animation.set_speed(-playing_animation.speed());
            } else {

            }
        }
    }
}