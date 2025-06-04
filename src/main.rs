use bevy::{platform::collections::HashMap, prelude::*};

// fn far_object(mut myquer: Query<&mut ScreenPosition>) {
//     for mut screenpos in myquer {
//         screenpos.0[0] = 2.0;
//         screenpos.0[1] = 1.0;
//         println!("Pos: {}",screenpos.0);
//     }
// }

#[derive(Resource)]
struct PreloadedAssets {
    images: HashMap<String,Handle<Image>>
}

fn init_menu_and_game(mut commands: Commands, server: Res<AssetServer>, mut preloaded_assets: ResMut<PreloadedAssets>) {
    
    commands.spawn(Camera2d);
    
    preloaded_assets.images.insert("fartcraftbackground.png".into(), server.load("fartcraftbackground.png"));

    commands.spawn((
        Sprite::from_image(preloaded_assets.images.get("fartcraftbackground.png").unwrap().clone()),
        Transform::from_xyz(50., 50., 50.)
    ));
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum MyAppState {
    #[default]
    Menu,
    WatchingTV,
    OutsideHouse,
    StreetProtest,
    GasStation,
    Downtown,
    HogCokeStore,
    DowntownAgain,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(OnEnter(MyAppState::Menu), init_menu_and_game)
        .add_systems(Update, asset_state)
        .init_state::<MyAppState>()
        .insert_resource(PreloadedAssets { images: HashMap::new() })
        .run();
}

fn asset_state(mysprite: Query<&Sprite>, server: Res<AssetServer>) {
    println!("{:?}",server.get_load_state(mysprite.single().unwrap().image.id()));
}
fn cleanup_system<T: Component>(
    mut commands: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}