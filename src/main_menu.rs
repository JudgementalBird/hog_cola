use bevy::{prelude::*, window::PrimaryWindow};
use crate::{transition::StartTransition, util::ScaledRelativeToWindow, BelongsTo, MyAppState};

pub(crate) struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_systems(OnEnter(MyAppState::Menu), (
                init_menu,
            ))
            .add_systems(Update, transition_if_start_button_clicked);
    }
}

#[derive(Component)]
struct MainMenuButton;
pub(crate) fn init_menu(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(server.load("images/fartcraftbackground.png")),
        Transform::from_xyz(0., 0., 0.),
        ScaledRelativeToWindow::BothAxes(1.0),
        BelongsTo(MyAppState::Menu)
    ));
    commands.spawn((
        Sprite::from_image(server.load("images/start button.png")),
        Transform::from_xyz(0., 0., 1.),
        ScaledRelativeToWindow::ByWidth(1./8.),
        MainMenuButton,
        BelongsTo(MyAppState::Menu)
    ));
}
fn transition_if_start_button_clicked(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    sprite_query: Query<(&Transform, &Sprite), With<MainMenuButton>>,
    assets: Res<Assets<Image>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands
) {
    if buttons.just_released(MouseButton::Left) {
        if let Ok((transform, sprite)) = sprite_query.single() {
            let image_dimensions = assets.get(sprite.image.id()).unwrap().size();
            let scaled_image_dimension = image_dimensions.as_vec2() * transform.scale.truncate();
            let bounding_box = Rect::from_center_size(transform.translation.truncate(), scaled_image_dimension);
    
            let mouse_pos = q_windows.single().unwrap().cursor_position().expect("Mouse pos unavailable :(") - Vec2 { x: 1280./2., y: 720./2. };
    
            if bounding_box.contains(mouse_pos) {
                commands.trigger(StartTransition);
                //println!("Triggered `StartTransition` from `transition_if_start_button_clicked`!");
            }
        }
    }
}