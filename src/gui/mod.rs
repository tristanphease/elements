use bevy::{input_focus::InputFocus, prelude::*};

use crate::{
    gui::{
        button::{button_system, create_button},
        gui_menu::{gui_menu_system, DebugMenu, GuiMenu},
        main_menu::{close_main_menu, setup_main_menu, start_button_menu_system},
    },
    AppState,
};

mod button;
mod gui_menu;
mod main_menu;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>();
        app.add_systems(Startup, setup_gui);
        app.add_systems(OnEnter(AppState::Menu), setup_main_menu);
        app.add_systems(
            Update,
            start_button_menu_system.run_if(in_state(AppState::Menu)),
        );
        app.add_systems(OnExit(AppState::Menu), close_main_menu);
        app.add_systems(Update, button_system);
        app.add_systems(Update, gui_menu_system);
    }
}

trait ButtonMenuComponent: Component + Copy {
    fn to_str(&self) -> &str;
}

pub(super) fn setup_gui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(20)),
            ..default()
        },
        children![create_button(GuiMenu), create_button(DebugMenu)],
    ));
}
