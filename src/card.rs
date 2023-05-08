
use bevy::prelude::*;

pub const CARD_TEXTURE_PATH: &str = "cards/encounters/card.png";

pub fn setup_card_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    // Load the card texture and spawn the card entity
    let texture_handle = asset_server.load(CARD_TEXTURE_PATH);

    commands.spawn(SpriteBundle {
        texture: texture_handle,
        ..Default::default()
    });
}