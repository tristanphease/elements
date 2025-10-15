//! Menu

use bevy::prelude::*;

use crate::{gui::ButtonMenuComponent, AppState};

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

// for opening debug related things
#[derive(Component, Clone, Copy)]
pub(super) struct DebugMenu;

impl ButtonMenuComponent for DebugMenu {
    fn to_str(&self) -> &str {
        "Debug"
    }
}
