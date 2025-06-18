use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{util::{BelongsTo, ScaledRelativeToWindow}, MyAppState};

pub(crate) struct OutsideHouse;

impl Plugin for OutsideHouse {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MyAppState::OutsideHouse), init_outside_house);
            // .add_systems(Update, (
            //     // transition_to_next.run_if(input_just_pressed(MouseButton::Left))
                
            // ).run_if(in_state(MyAppState::OutsideHouse)));
    }
}

fn init_outside_house(
    server: Res<AssetServer>,
    mut commands: Commands
) {
    commands.spawn((
        BelongsTo(MyAppState::OutsideHouse),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode::new(server.load("images/outside_house/background_bleak.png")),
    ));
}