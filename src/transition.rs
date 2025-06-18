use bevy::prelude::*;

use crate::{util::{BelongsTo, ScaledRelativeToWindow, Z_LEVEL_UI}, MyAppState};

pub struct Transition;

impl Plugin for Transition {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StartTransition>()
            .init_resource::<TransitionState>()
            .add_observer(transition_starter)
            .add_systems(Update, transitioner)
            .add_observer(level_despawner);
    }
}

#[derive(Resource, Debug)]
enum TransitionState {
    None,
    FadeToBlack(Timer),
    FadeFromBlack(Timer),
}
impl Default for TransitionState {
    fn default() -> Self {
        TransitionState::None
    }
}

#[derive(Component)]
struct BlackTransitionEntity;

#[derive(Event)]
pub struct StartTransition;
#[derive(Event)]
struct LevelDespawn(MyAppState);


fn transition_starter(
    _: Trigger<StartTransition>,
    mut transition_state: ResMut<TransitionState>,
    mut commands: Commands,
    server: Res<AssetServer>
) {
    match *transition_state {
        TransitionState::None => {
            
            *transition_state = TransitionState::FadeToBlack(Timer::from_seconds(1., TimerMode::Once));
            
            let black_square_handle = server.load("images/black square lol.png");

            let mut black_square_sprite = Sprite::from_image(black_square_handle.clone());
            black_square_sprite.color.set_alpha(0.);
            commands.spawn((
                black_square_sprite,
                Transform::from_xyz(0., 0., 1.),
                ScaledRelativeToWindow::BothAxes(1.0),
                BlackTransitionEntity
            ));

            let mut black_square_imagenode = ImageNode::new(black_square_handle);
            black_square_imagenode.color.set_alpha(0.);
            commands.spawn((
                BlackTransitionEntity,
                ZIndex(Z_LEVEL_UI+10),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                black_square_imagenode,
            ));
        }
        _ => {}
    };
}

fn transitioner(
    mut transition_state: ResMut<TransitionState>,
    time: Res<Time>,
    mut black_transition_boxes: Query<(Entity, Option<&mut Sprite>, Option<&mut ImageNode>), With<BlackTransitionEntity>>,
    mut commands: Commands,
    state: Res<State<MyAppState>>,
    mut nextstate: ResMut<NextState<MyAppState>>,
) {
    match &mut *transition_state {
        TransitionState::None => {},
        TransitionState::FadeToBlack(timer) => {

            timer.tick(time.delta());

            for (_, maybe_sprite, maybe_imagenode) in black_transition_boxes.iter_mut() {
                if let Some(mut sprite) = maybe_sprite {
                    let a = start_easing(1. - timer.fraction_remaining());
                    sprite.color.set_alpha(a);
                }
                if let Some(mut imagenode) = maybe_imagenode {
                    let a = start_easing(1. - timer.fraction_remaining());
                    imagenode.color.set_alpha(a);
                }
            }
            
            if timer.finished() {
                *transition_state = TransitionState::FadeFromBlack(Timer::from_seconds(1.45, TimerMode::Once));
                commands.trigger(LevelDespawn(state.get().clone()));
                nextstate.set(state.next());
            }
        },
        TransitionState::FadeFromBlack(timer) => {

            timer.tick(time.delta());

            if timer.finished() {
                *transition_state = TransitionState::None;
                for (entity, maybe_sprite, maybe_imagenode) in black_transition_boxes.iter_mut() {
                    if let Some(_) = maybe_sprite {
                        commands.entity(entity).despawn();
                    }
                    if let Some(_) = maybe_imagenode {
                        commands.entity(entity).despawn();
                    }
                }
            } else {
                for (_, maybe_sprite, maybe_imagenode) in black_transition_boxes.iter_mut() {
                    if let Some(mut sprite) = maybe_sprite {
                        let a = end_easing(timer.fraction_remaining(), 0.45);
                        sprite.color.set_alpha(a);
                    }
                    if let Some(mut imagenode) = maybe_imagenode {
                        let a = end_easing(timer.fraction_remaining(), 0.45);
                        imagenode.color.set_alpha(a);
                    }
                }
            }
        },
    }
}
fn level_despawner(
    trigger: Trigger<LevelDespawn>,
    entities: Query<(Entity, &BelongsTo)>,
    mut commands: Commands,
) {
    let a = trigger.event().0;
    for (entity, belongs_to) in entities {
        if belongs_to.0 == a {
            commands.entity(entity).despawn();
        }
    }
}

// should probably use bevy tween instead but why pull in another dependency when I can do it simple and good enough 🦑
fn start_easing(x: f32) -> f32 {
    if x < 0.5 {
        (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}
fn end_easing(x: f32, stay_black_for: f32) -> f32 {
    let mut x = x;
    if x > (1.-stay_black_for) {
        return 1.;
    } else {
        x = rescale(x,0.,1.-stay_black_for,0.,1.);
        if x < 0.5 {
            return (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
        } else {
            return ((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
        }
    }
}
fn rescale(x:f32, min:f32, max:f32, a:f32, b:f32) -> f32 {
    (b-a)*(x-min) / (max-min)+a
}