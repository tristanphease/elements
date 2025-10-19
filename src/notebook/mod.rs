mod page;

use std::time::Duration;

use bevy::{gltf::GltfMeshName, prelude::*};

use crate::{
    drawable::Drawable,
    scene_hook::{HookedSceneBundle, SceneHook},
};

const NOTEBOOK_PATH: &str = "models/notebook.glb";

pub fn add_notebook_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(NOTEBOOK_PATH));
    // commands.spawn(SceneRoot(scene));
    commands.spawn(HookedSceneBundle {
        scene: SceneRoot(scene),
        hook: SceneHook::new(|entity, cmds| {
            match entity.get::<GltfMeshName>().map(|x| x.0.as_str()) {
                Some("page_mesh") => cmds.insert(Drawable::default()),
                _ => cmds,
            };
        }),
    });

    // add animations
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(NOTEBOOK_PATH))
    ]);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(NotebookAnimations {
        animations: node_indices,
        graph: graph_handle,
    });
}

pub fn setup_notebook_animations_once_loaded(
    mut commands: Commands,
    animations: Res<NotebookAnimations>,
    mut anim_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    // when animation player is added, play notebook animations on the entity
    for (entity, mut player) in &mut anim_players {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .pause();

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
    for (mut player, mut _transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
            continue;
        };

        if keyboard_input.just_pressed(KeyCode::Space) {
            let playing_animation_option = player.animation_mut(playing_animation_index);

            if let Some(playing_animation) = playing_animation_option {
                if playing_animation.is_paused() {
                    playing_animation.resume();
                }
                if playing_animation.is_finished() {
                    let current_time = playing_animation.seek_time();
                    playing_animation.replay();
                    playing_animation.set_seek_time(current_time);
                }
                playing_animation.set_speed(-playing_animation.speed());
            }
        }
    }
}
