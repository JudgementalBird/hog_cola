use bevy::prelude::*;

pub(crate) const Z_LEVEL_UI: i32 = 11;

#[derive(Component)]
pub(crate) struct BelongsTo(pub(crate) MyAppState);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub(crate) enum MyAppState {
    #[default]
    Menu,
    ExpositionBedroom,
    WatchingTV,
    OutsideHouse,
    StreetProtest,
    GasStation,
    Downtown,
    HogColaStore,
    DowntownAgain,
    Over,
}
impl MyAppState {
    pub(crate) fn next(&self) -> Self {
        return match self {
            MyAppState::Menu => MyAppState::ExpositionBedroom,
            MyAppState::ExpositionBedroom => MyAppState::WatchingTV,
            MyAppState::WatchingTV => MyAppState::OutsideHouse,
            MyAppState::OutsideHouse => MyAppState::StreetProtest,
            MyAppState::StreetProtest => MyAppState::GasStation,
            MyAppState::GasStation => MyAppState::Downtown,
            MyAppState::Downtown => MyAppState::HogColaStore,
            MyAppState::HogColaStore => MyAppState::DowntownAgain,
            MyAppState::DowntownAgain => MyAppState::Over,
            MyAppState::Over => MyAppState::Over,
        }
    }
}

#[derive(Component)]
pub(crate) enum ScaledRelativeToWindow {
    BothAxes(f32),
    ByWidth(f32),
}

