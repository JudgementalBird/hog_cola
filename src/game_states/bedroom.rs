use std::{collections::VecDeque};
use bevy::{prelude::*};

use crate::{util::{BelongsTo, Z_LEVEL_UI}, MyAppState};

pub(crate) struct Bedroom;

impl Plugin for Bedroom {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MyAppState::ExpositionBedroom), init_bedroom)
            .add_systems(Update, (
                advance_dialogue
            ).run_if(in_state(MyAppState::ExpositionBedroom)));
    }
}

#[derive(Component)]
struct BedroomBlackSquare;

fn init_bedroom(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    commands.spawn((
        BelongsTo(MyAppState::ExpositionBedroom),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode::new(server.load("images/bedroom/bedroom_background.png")),
    ));

    let mut black_square_imagenode = ImageNode::new(server.load("images/black square lol.png"));
    black_square_imagenode.color.set_alpha(1.);
    commands.spawn((
        BelongsTo(MyAppState::ExpositionBedroom),
        ZIndex(Z_LEVEL_UI+1),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        black_square_imagenode,
        BedroomBlackSquare,
    ));
    let monologue = [
            "Mr. Shit, a man in his late 20s, lived a straightforward and honest life.",
            "He knew not to worry too hard, and that hard work paid off.",
            "One day, Mr. Shit woke up feeling,",
            "A Little Off...",
        ];
    commands.spawn((
        BelongsTo(MyAppState::ExpositionBedroom),
        ZIndex(Z_LEVEL_UI+2),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            BelongsTo(MyAppState::ExpositionBedroom),
            Text::new(monologue[0]),
            TextFont {
                font_size: 27.,
                ..default()
            },
            BedroomExpositionText,
        )]
    ));
    let mut monologue = VecDeque::from(monologue);
    monologue.pop_front();
    commands.insert_resource(
        BlackExpositionState::Monologue(
            DialogueState::Text( Timer::from_seconds(0.25, TimerMode::Once) ),
        monologue)
    );
}

#[derive(Component)]
struct BedroomExpositionText;

#[derive(Resource)]
enum BlackExpositionState {
    Monologue(DialogueState, VecDeque<&'static str>),
    FadeFromExposition(Timer),
    Done
}
enum DialogueState {
    Text(Timer),
    FadeTextToBlack(Timer),
    FadeBlackToText(Timer),
}

fn advance_dialogue(
    mut text: Query<(&mut Text, &mut TextColor), With<BedroomExpositionText>>,
    mut squar: Query<&mut ImageNode, With<BedroomBlackSquare>>,
    mut exposition_state: ResMut<BlackExpositionState>,
    time: Res<Time>,
) {
    let (mut text, mut text_color) = text.single_mut().unwrap();
    let mut square_node = squar.single_mut().unwrap();

    match &mut *exposition_state {
        BlackExpositionState::Monologue(dialoguestate, monologue) => {
            match dialoguestate {
                DialogueState::Text(timer) => {
                    if timer.tick(time.delta()).finished() {
                        *dialoguestate = DialogueState::FadeTextToBlack(Timer::from_seconds(0.14, TimerMode::Once))
                    }
                },
                DialogueState::FadeTextToBlack(timer) => {
                    timer.tick(time.delta());
                    if !timer.finished() {
                        text_color.0.set_alpha(timer.fraction_remaining());
                        return;
                    }
                    match monologue.get(0) {
                        Some(_) => {
                            text.clear();
                            text.push_str(monologue[0]);
                            monologue.pop_front();
                            *dialoguestate = DialogueState::FadeBlackToText(Timer::from_seconds(0.14, TimerMode::Once));
                        },
                        None => {
                            *exposition_state = BlackExpositionState::FadeFromExposition(Timer::from_seconds(0.24, TimerMode::Once));
                        },
                    }
                },
                DialogueState::FadeBlackToText(timer) => {
                    timer.tick(time.delta());
                    if !timer.finished() {
                        text_color.0.set_alpha(timer.fraction());
                        return;
                    }
                    *dialoguestate = DialogueState::Text(Timer::from_seconds(0.25, TimerMode::Once))
                },
            };
        },
        BlackExpositionState::FadeFromExposition(timer) => {
            timer.tick(time.delta());
            square_node.color.set_alpha(timer.fraction_remaining());
            if timer.finished() {
                *exposition_state = BlackExpositionState::Done;
            }
        },
        BlackExpositionState::Done => {
            
        },
    };
}