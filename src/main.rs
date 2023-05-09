use bevy::prelude::*;
use card::setup_card_system;
use crate::move_to_center_system::move_to_center_system;
use crate::card_selection_system::card_selection_system;

pub mod card;
pub mod deck;
pub mod card_halo;
pub mod move_to_center_system;
pub mod card_selection_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_card_system)
        .add_system(move_to_center_system)
        .add_system(card_selection_system)
        .run();
}
