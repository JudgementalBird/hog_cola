use bevy::{prelude::*, window::WindowResolution};
use crate::{bedroom::Bedroom, main_menu::MainMenu, transition::{Transition, TransitionState}, util::scale_sprites_relative_to_window};

// fn far_object(mut myquer: Query<&mut ScreenPosition>) {
//     for mut screenpos in myquer {
//         screenpos.0[0] = 2.0;
//         screenpos.0[1] = 1.0;
//         println!("Pos: {}",screenpos.0);
//     }
// }

/// This is my start to a point and click bevy game, at this point I feel it is very messy to work on. Starting to wonder if maybe ECS is not a suitable for an old flash point and click style game.
/// Some pain points:
/// -   I want to preload everything on startup so that we don't have to wait for assets to pop in as e.g. entities with sprites get spawned.
///     I chose to implement this as a PreloadedAssets struct with a hashmap for each type of asset (image, audio, etc).
///     It is kinda clunky and logically coupled in a bad way to have to remember to add the path for an asset first to the proper part of PreloadedAssets, and then to the place where I spawn it.
///     One solution might be a preloader of some sort that recursively loads everything in the assets/ folder on startup, and makes it easily available by path or name from a system.
/// 
/// -   I forced the size of the window to 1280p so I can hardcode buttons and sprite positions. Optimally it would be adaptive, I think I'm gonna lock the aspect ratio, figure out a way to set a minimum size, and switch to bevy's nodes/flexbox ui.
/// 
/// -   Writing systems that accomplish something across multiple calls/game ticks over some amount of time is confusing and hard. Maybe Events can save me? E.g:
///     // Assuming there is a run condition so this only runs if the mouse is clicked
///     fn walk_character_to_place( mouse pos query, character query,  ) { // using a marker component to get specifically the one character
///         
///         
///         
///     }
/// 
/// 
/// 
/// IDEAS:
/// As a general rule, every scene in its own plugin.
/// 
/// UI in its own plugin?
/// 
/// ONE despawn system, using ONE BelongsTo struct, which stores a game state enum, and gets put on everything spawned in a state
/// 
/// Empathy implemented using events?
/// 
/// Transition implemented firstly as the cause of the transition sending a transition event, which causes UI to stop responding (UI functions set a bool resource? local resource? or UI systems read those events every update?), 
/// and causes fade to black. Fade to black will be a system that runs every update, storing if the event has happened somehow (same as above prob).. spawns black sprite, fades in opacity.
/// Maybe UI system and fade to black can share a "transitioning" resource that has all both need? Should this design be improved, if so, to what?
/// When the fade system has made the black covering the screen fully opaque, it invokes another transition event that contains from and to
/// 

mod main_menu;
mod loading;
mod bedroom;
mod util;
mod transition;
mod watching_tv;

#[derive(Component)]
struct BelongsTo(MyAppState);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum MyAppState {
    Loading,
    #[default]
    Menu,
    ExpositionBedroom,
    WatchingTV,
    OutsideHouse,
    StreetProtest,
    GasStation,
    Downtown,
    HogCokeStore,
    DowntownAgain,
    Over,
}
impl MyAppState {
    fn next(&self) -> Self {
        return match self {
            MyAppState::Loading => MyAppState::Menu,
            MyAppState::Menu => MyAppState::ExpositionBedroom,
            MyAppState::ExpositionBedroom => MyAppState::WatchingTV,
            MyAppState::WatchingTV => MyAppState::OutsideHouse,
            MyAppState::OutsideHouse => MyAppState::StreetProtest,
            MyAppState::StreetProtest => MyAppState::GasStation,
            MyAppState::GasStation => MyAppState::Downtown,
            MyAppState::Downtown => MyAppState::HogCokeStore,
            MyAppState::HogCokeStore => MyAppState::DowntownAgain,
            MyAppState::DowntownAgain => MyAppState::Over,
            MyAppState::Over => MyAppState::Over,
        }
    }
}

#[derive(Resource)]
struct GameState {
    transition_state: TransitionState
}
impl Default for GameState {
    fn default() -> Self {
        Self { transition_state: TransitionState::None }
    }
}

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
        .add_systems(Update,
            scale_sprites_relative_to_window,
        )
        .add_plugins((
            // Loading,
            Transition,
            MainMenu,
            Bedroom,
            
        ))
        .init_state::<MyAppState>()
        .init_resource::<GameState>()
        .run();
        
        // .add_systems(Update,
        //     (
        //         begin_transition_menu_to_bedroom.run_if(mouse_pressed_over_main_mouse_button),
        //         half_finish_transition_menu_to_bedroom.after(begin_transition_menu_to_bedroom)
        //     ).run_if(in_state(MyAppState::Menu)))
}






// fn asset_state(mysprite: Query<&Sprite>, server: Res<AssetServer>) {
//     println!("{:?}",server.get_load_state(mysprite.single().unwrap().image.id()));
// }

// fn cleanup_system<T: Component>(
//     mut commands: Commands,
//     q: Query<Entity, With<T>>,
// ) {
//     for e in q.iter() {
//         commands.entity(e).despawn();
//     }
// }