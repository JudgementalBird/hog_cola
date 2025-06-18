use bevy::prelude::*;
use crate::{transition::StartTransition, MyAppState};

pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MyAppState::ExpositionBedroom), add_ui)
            .add_systems(Update, button_system);
    }
}

fn add_ui(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    commands.spawn((
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        ImageNode::new(server.load("images/next_placeholder.png"))
    ));
}

fn button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_buttons.just_released(MouseButton::Left) {
        return;
    }
    if let Ok(_) = interaction_query.single() {
        commands.trigger(StartTransition);
    }
}