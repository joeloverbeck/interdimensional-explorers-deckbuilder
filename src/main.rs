use bevy::prelude::*;
use card::setup_card_system;

pub mod card;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_card_system)
        .run();
}
