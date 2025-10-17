use bevy::{input_focus::InputFocus, prelude::*};

use crate::{
    gui::{
        button::{button_system, create_button},
        gui_menu::{
            clear_image_button_system, close_debug_menu, debug_menu_system, gui_menu_system,
            save_image_button_system, setup_debug_menu, DebugMenu, GuiMenu, GuiMenuState,
        },
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
        // general stuff
        app.init_resource::<InputFocus>();
        app.add_systems(Update, button_system);
        // main menu stuff
        app.add_systems(OnEnter(AppState::Menu), setup_main_menu);
        app.add_systems(
            Update,
            start_button_menu_system.run_if(in_state(AppState::Menu)),
        );
        app.add_systems(OnExit(AppState::Menu), close_main_menu);
        // gui menu stuff
        app.init_state::<GuiMenuState>();
        app.add_systems(OnEnter(GuiMenuState::Debug), setup_debug_menu);
        app.add_systems(OnExit(GuiMenuState::Debug), close_debug_menu);
        app.add_systems(Startup, setup_gui);
        app.add_systems(Update, gui_menu_system);
        app.add_systems(Update, debug_menu_system);
        app.add_systems(Update, save_image_button_system);
        app.add_systems(Update, clear_image_button_system);
    }
}

#[derive(Resource)]
pub(super) struct GuiMenuData {
    gui_menu_entity: Entity,
}

trait ButtonMenuComponent: Component + Copy {
    fn to_str(&self) -> &str;
}

pub(super) fn setup_gui(mut commands: Commands) {
    let gui_menu_entity = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::RowReverse,
                padding: UiRect::all(px(20)),
                ..default()
            },
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![create_button(GuiMenu), create_button(DebugMenu)]
            )],
        ))
        .id();
    commands.insert_resource(GuiMenuData { gui_menu_entity });
}
