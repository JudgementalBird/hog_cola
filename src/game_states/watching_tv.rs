use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{util::{BelongsTo, ScaledRelativeToWindow}, MyAppState};

pub(crate) struct WatchingTV;

impl Plugin for WatchingTV {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MyAppState::WatchingTV), init_watching_tv);
            // .add_systems(Update, (
            //     transition_to_next.run_if(input_just_pressed(MouseButton::Left))
            // ).run_if(in_state(MyAppState::WatchingTV)));
    }
}

fn init_watching_tv(
    server: Res<AssetServer>,
    mut commands: Commands
) {
    commands.spawn((
        BelongsTo(MyAppState::WatchingTV),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode::new(server.load("images/watching_tv/television_off.png")),
    ));
}