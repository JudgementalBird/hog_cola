use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct Inventory(Vec<Item>);
impl Default for Inventory {
    fn default() -> Self {
        Self(Vec::new())
    }
}

pub(crate) enum Item {
    Poster,
    Bottle(Fill),
    Wallet(usize),
}
pub(crate) enum Fill {
    Filled,
    Empty
}