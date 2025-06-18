use bevy::prelude::*;

pub struct EmpathyPlugin;

impl Plugin for EmpathyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Empathy>()
            .add_observer(receive_empathy_event);
    }
}

#[derive(Resource)]
pub struct Empathy(f32);
impl Default for Empathy {
    fn default() -> Self {
        Self(0.0)
    }
}

#[derive(Event)]
pub struct EmpathyEvent(f32);

fn receive_empathy_event(
    empathy_event: Trigger<EmpathyEvent>,
    mut empathy_resource: ResMut<Empathy>
) {
    empathy_resource.0 += empathy_event.0;
}