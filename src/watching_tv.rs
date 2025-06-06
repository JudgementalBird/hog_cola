use bevy::prelude::*;

use crate::MyAppState;

struct WatchingTV;

impl Plugin for WatchingTV {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MyAppState::WatchingTV), init_watching_tv);
    }
}

fn init_watching_tv() {

}