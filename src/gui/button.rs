//! Generic stuff for handling the button systems on the GUI
//!
//!

use bevy::{input_focus::InputFocus, prelude::*};

use crate::gui::ButtonMenuComponent;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.25, 0.3);

const BORDER_BUTTON: Color = Color::BLACK;

pub(super) fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut bg_color, mut border_color, mut button) in &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *bg_color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(Color::srgb(0.05, 0.0, 0.2));

                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                *bg_color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::srgb(0.25, 0.2, 0.4));
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *bg_color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(BORDER_BUTTON);
            }
        }
    }
}

pub(super) fn create_button(menu: impl ButtonMenuComponent) -> impl Bundle {
    (
        Button,
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(px(10)),
            ..default()
        },
        BorderRadius::all(px(5)),
        BorderColor::all(BORDER_BUTTON),
        BackgroundColor(Color::BLACK),
        menu,
        children![(
            Text::new(menu.to_str()),
            TextFont::default(),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}
