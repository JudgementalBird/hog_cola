use bevy::prelude::*;

#[derive(Resource)]
struct CurrentInteraction(InteractionStateVariant);

enum InteractionStateVariant {
    None,
    Dialogue_InteractionState( Dialogue_InteractionState ),
    SpecificDialogue_InteractionState( SpecificDialogue_InteractionState ),
    Dialogue_YesNo_SpecificDialogue_InteractionState( Dialogue_YesNo_SpecificDialogue_InteractionState ),
    Dialogue_MaybeYesNo_SpecificDialogue( Dialogue_MaybeYesNo_SpecificDialogue_InteractionState ),
}

enum Dialogue_InteractionState { // able to trigger pick up item event
    Dialogue(Timer)
}
enum SpecificDialogue_InteractionState { // just dialogue
    SpecificDialogue(Timer)
}
enum Dialogue_YesNo_SpecificDialogue_InteractionState { // able to trigger morality event and trigger pick up item event
    Dialogue(Timer),
    YesNo,
    SpecificDialogue(Timer),
}
enum Dialogue_MaybeYesNo_SpecificDialogue_InteractionState { // able to trigger morality event, trigger pick up item event, and trigger go to specific scene event
    Dialogue(Timer),
    MaybeYesNo,
    SpecificDialogue(Timer),
}