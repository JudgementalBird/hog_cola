use bevy::prelude::*;
use crate::{util::ScaledRelativeToWindow, BelongsTo, MyAppState};

pub(crate) struct Bedroom;

impl Plugin for Bedroom {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MyAppState::ExpositionBedroom), init_bedroom);
    }
}

fn init_bedroom(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    println!("init_bedroom called from `OnEnter(MyAppState::ExpositionBedroom)` schedule!");
    commands.spawn((
        Sprite::from_image(server.load("images/tvbedroomsomething.png")),
        Transform::from_xyz(0., 0., 0.),
        ScaledRelativeToWindow::BothAxes(1.0),
        BelongsTo(MyAppState::ExpositionBedroom)
    ));
}