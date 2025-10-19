use bevy::prelude::*;

use crate::{
    gui::{create_button, ButtonMenuComponent},
    AppState,
};

pub(super) fn start_button_menu_system(
    interaction_query: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            // start game
            next_state.set(AppState::Playing);
        }
    }
}

#[derive(Component, Clone, Copy)]
pub(super) struct StartButton;

impl ButtonMenuComponent for StartButton {
    fn to_str(&self) -> &str {
        "Start"
    }
}

#[derive(Resource)]
pub(super) struct MainMenuData {
    menu_entity: Entity,
}

pub(super) fn setup_main_menu(mut commands: Commands) {
    let menu_entity = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![create_button(StartButton)],
        ))
        .id();
    commands.insert_resource(MainMenuData { menu_entity });
}

pub(super) fn close_main_menu(mut commands: Commands, main_menu_data: Res<MainMenuData>) {
    commands.entity(main_menu_data.menu_entity).despawn();
}
