use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use crate::{empathy::EmpathyPlugin, game_states::{bedroom::Bedroom, main_menu::MainMenu, outside_house::OutsideHouse, watching_tv::WatchingTV}, inventory::Inventory, transition::Transition, ui::Ui, util::MyAppState};

mod interaction;
mod util;
mod ui;
mod transition;
mod inventory;
mod empathy;

mod game_states;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // prevents blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        resolution: WindowResolution::new(1280., 720.).with_scale_factor_override(1.),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
        ) 
        .add_plugins((
            Ui,
            Transition,
            EmpathyPlugin,
            MainMenu,
            Bedroom,
            WatchingTV,
            OutsideHouse,
            EguiPlugin { enable_multipass_for_primary_context: true },
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .init_state::<MyAppState>()
        .init_resource::<Inventory>()
        .run();
}