use bevy::prelude::*;
use crate::{util::{BelongsTo, ScaledRelativeToWindow, Z_LEVEL_UI}, MyAppState};

pub(crate) struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_systems(OnEnter(MyAppState::Menu), (
                init_menu,
            ));
            // .add_systems(Update, transition_to_next.run_if(input_just_pressed(MouseButton::Left)));
            // .add_systems(Update, transition_if_start_button_clicked.run_if(input_just_pressed(MouseButton::Left)));
    }
}

#[derive(Component)]
struct MainMenuButton;
pub(crate) fn init_menu(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        BelongsTo(MyAppState::Menu),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode::new(server.load("images/menu_placeholder.png")),
    ));
    commands.spawn((
        BelongsTo(MyAppState::Menu),
        ZIndex(Z_LEVEL_UI),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            // BelongsTo(MyAppState::Menu),
            Button,
            Node {
                padding: UiRect {
                    bottom: Val::Px(5.5),
                    top: Val::Px(5.5),
                    left: Val::Px(30.2),
                    right: Val::Px(30.2),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(server.load("images/button_placeholder.png")),
            children![(
                // BelongsTo(MyAppState::Menu),
                Text::new("START THAT SHIT!!"),
                TextFont {
                    font_size: 41.6,
                    ..default()
                },
            )],
        )],
    ));
}

