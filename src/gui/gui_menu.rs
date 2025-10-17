//! Menu

use bevy::prelude::*;

use crate::{
    drawable::{ClearDrawableImage, SaveDrawableImage},
    gui::{create_button, ButtonMenuComponent, GuiMenuData},
    AppState,
};

#[derive(Component, Clone, Copy)]
pub(super) struct GuiMenu;

impl ButtonMenuComponent for GuiMenu {
    fn to_str(&self) -> &str {
        "Menu"
    }
}

pub(super) fn gui_menu_system(
    interaction_query: Query<&Interaction, (With<GuiMenu>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in interaction_query {
        match interaction {
            Interaction::Pressed => {
                // open menu
                next_state.set(AppState::Menu);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, States)]
pub(super) enum GuiMenuState {
    #[default]
    Empty,
    Debug,
}

// for opening debug related things
#[derive(Component, Clone, Copy)]
pub(super) struct DebugMenu;

impl ButtonMenuComponent for DebugMenu {
    fn to_str(&self) -> &str {
        "Debug"
    }
}

pub(super) fn debug_menu_system(
    interaction_query: Query<&Interaction, (With<DebugMenu>, Changed<Interaction>)>,
    current_state: Res<State<GuiMenuState>>,
    mut next_state: ResMut<NextState<GuiMenuState>>,
) {
    for interaction in interaction_query {
        match interaction {
            Interaction::Pressed => {
                let new_state = if *current_state.get() == GuiMenuState::Debug {
                    GuiMenuState::Empty
                } else {
                    GuiMenuState::Debug
                };
                next_state.set(new_state);
            }
            _ => {}
        }
    }
}

// for opening debug related things

#[derive(Component, Clone, Copy)]
pub(super) struct SaveImageButton;

impl ButtonMenuComponent for SaveImageButton {
    fn to_str(&self) -> &str {
        "Save Image"
    }
}

pub(super) fn save_image_button_system(
    interaction_query: Query<&Interaction, (With<SaveImageButton>, Changed<Interaction>)>,
    mut save_drawable_image_writer: MessageWriter<SaveDrawableImage>,
) {
    for interaction in interaction_query {
        match interaction {
            Interaction::Pressed => {
                save_drawable_image_writer.write(SaveDrawableImage);
            }
            _ => {}
        }
    }
}

#[derive(Component, Clone, Copy)]
pub(super) struct ClearImageButton;

impl ButtonMenuComponent for ClearImageButton {
    fn to_str(&self) -> &str {
        "Clear Image"
    }
}

pub(super) fn clear_image_button_system(
    interaction_query: Query<&Interaction, (With<ClearImageButton>, Changed<Interaction>)>,
    mut clear_drawable_image_writer: MessageWriter<ClearDrawableImage>,
) {
    for interaction in interaction_query {
        match interaction {
            Interaction::Pressed => {
                clear_drawable_image_writer.write(ClearDrawableImage);
            }
            _ => {}
        }
    }
}

#[derive(Resource)]
pub(super) struct DebugMenuData {
    debug_menu_entity: Entity,
}

pub(super) fn setup_debug_menu(mut commands: Commands, gui_menu_data: Res<GuiMenuData>) {
    //
    let debug_menu_entity = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                create_button(SaveImageButton),
                create_button(ClearImageButton)
            ],
        ))
        .id();
    commands
        .entity(gui_menu_data.gui_menu_entity)
        .add_child(debug_menu_entity);
    commands.insert_resource(DebugMenuData { debug_menu_entity });
}

pub(super) fn close_debug_menu(mut commands: Commands, debug_menu_data: Res<DebugMenuData>) {
    commands.entity(debug_menu_data.debug_menu_entity).despawn();
}
