use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{util::ScaledRelativeToWindow, BelongsTo, GameState, MyAppState};

pub(crate) struct Transition;

impl Plugin for Transition {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StartTransition>()
            .add_observer(transition_starter)
            .add_systems(Update, transitioner)
            .add_observer(level_despawner);
    }
}


pub(crate) enum TransitionState {
    None,
    FadeToBlack(Timer),
    FadeFromBlack(Timer),
}
#[derive(Component)]
pub(crate) struct BlackTransitionEntity;

#[derive(Event)]
pub(crate) struct StartTransition;
#[derive(Event)]
pub(crate) struct LevelDespawn(MyAppState);


fn transition_starter(
    _: Trigger<StartTransition>,
    mut gamestate: ResMut<GameState>,
    mut commands: Commands,
    server: Res<AssetServer>
) {
    match gamestate.transition_state {
        TransitionState::None => {
            
            gamestate.transition_state = TransitionState::FadeToBlack(Timer::from_seconds(1., TimerMode::Once));
            
            let mut black_sprite = Sprite::from_image(server.load("images/black square lol.png"));
            black_sprite.color.set_alpha(0.);
            commands.spawn((
                black_sprite,
                Transform::from_xyz(0., 0., 1.),
                ScaledRelativeToWindow::BothAxes(1.0),
                BlackTransitionEntity
            ));
        }
        _ => {}
    };
    //println!("Transition start event received by observer `transition_starter` through trigger `Trigger<StartTransition>`")
}

fn transitioner(
    mut gamestate: ResMut<GameState>,
    time: Res<Time>,
    mut black_transition_entity: Query<(Entity, &mut Sprite), With<BlackTransitionEntity>>,
    mut commands: Commands,
    state: Res<State<MyAppState>>,
    mut nextstate: ResMut<NextState<MyAppState>>,
    
) {
    match &mut gamestate.transition_state {
        TransitionState::None => {},
        TransitionState::FadeToBlack(timer) => {
            let (_,mut sprite) = black_transition_entity.single_mut().unwrap();
            
            timer.tick(time.delta());
            sprite.color.set_alpha(start_easing(1. - timer.fraction_remaining()));
            
            if timer.just_finished() {
                gamestate.transition_state = TransitionState::FadeFromBlack(Timer::from_seconds(1.45, TimerMode::Once));
                commands.trigger(LevelDespawn(state.get().clone()));
                nextstate.set(state.next());
                println!("Set next state. This should fire all OnExit and OnEnter!")
            }
        },
        TransitionState::FadeFromBlack(timer) => {
            let (entity,mut sprite) = black_transition_entity.single_mut().unwrap();

            timer.tick(time.delta());
            sprite.color.set_alpha(end_easing(timer.fraction_remaining(), 0.45));
            
            if timer.just_finished() {
                gamestate.transition_state = TransitionState::None;
                commands.entity(entity).despawn();
                println!("Cleaned up black transition entity!")
            }
        },
    }
}
fn level_despawner(
    trigger: Trigger<LevelDespawn>,
    entities: Query<(Entity, &BelongsTo)>,
    mut commands: Commands,
) {
    for (entity, belongs_to) in entities {
        if belongs_to.0 == trigger.event().0 {
            commands.entity(entity).despawn();
        }
    }
}

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