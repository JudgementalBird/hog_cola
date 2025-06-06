// use std::any::Any;

// use bevy::{platform::collections::HashMap, prelude::*};
// use crate::{util::ScaledRelativeToWindow, BelongsTo, MyAppState};

// pub struct Loading;

// impl Plugin for Loading {
//     fn build(&self, app: &mut bevy::app::App) {
//         app
//             .insert_resource(PreloadedAssets(HashMap::new()))
//             .add_systems(OnEnter(MyAppState::Loading), init_game)
//             .add_systems(Update, (
//                 go_to_main_menu_when_done
//             ).run_if(in_state(MyAppState::Loading)));
//     }
// }

// #[derive(Resource)]
// pub(crate) struct PreloadedAssets {
//     loading_assets: HashMap<String,UntypedHandle>,
//     confirmation_frames_target: usize,
//     confirmation_frames_progress: usize,
// }

// fn init_game(mut commands: Commands, server: Res<AssetServer>, mut preloaded_assets: ResMut<PreloadedAssets>) {
//     let assets_to_load = [
//         "images/fartcraftbackground.png",
//         "images/start button.png",
//         "images/tvbedroomsomething.png",
//         "images/black square lol.png",

//         "sounds/alarm-clock-908676-FINAL.mp3",
//         ];
//     for path in assets_to_load {
//         preloaded_assets.loading_assets.insert(path.into(), server.load_untyped(path).untyped());
//     }
//     commands.spawn((
//         Sprite::from_image(preloaded_assets.loading_assets.get("images/loading.png").unwrap().clone().typed_debug_checked()),
//         Transform::from_xyz(0., 0., 0.),
//         ScaledRelativeToWindow::BothAxes(1.0),
//         BelongsTo(MyAppState::Loading)
//     ));
//     // // locked aspect ratio
//     // commands.spawn((
//     //     Camera2d,
//     //     Name::new("Camera"),
//     //     Projection::Orthographic(OrthographicProjection {
//     //         scaling_mode: bevy::render::camera::ScalingMode::AutoMin { min_width: 800., min_height: 450. },
//     //         ..OrthographicProjection::default_2d()
//     //     })
//     // ));
//     commands.spawn(Camera2d);
// }

// fn go_to_main_menu_when_done(
//     handles: Res<PreloadedAssets>,
//     server: Res<AssetServer>
// ) {
//     for (_, handle) in handles.loading_assets.iter() {

//     }
// }