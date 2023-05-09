use bevy::prelude::*;

use crate::{
    card_halo::{CardHalo, HALO_TEXTURE_PATH},
    deck::{load_encounter_cards, EncounterDeck},
};

pub const CARD_TEXTURE_PATH: &str = "cards/encounters/card.png";

pub fn setup_card_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load encounters from the "encounters.toml" file
    let encounters = load_encounter_cards();

    // Create a new Deck instance with the loaded encounters
    let deck = EncounterDeck { cards: encounters };

    // Load the card texture and spawn the card entity
    let texture_handle = asset_server.load(CARD_TEXTURE_PATH);

    let halo_texture_handle = asset_server.load(HALO_TEXTURE_PATH);

    let scale_factor = Vec3::new(0.5, 0.5, 1.0);

    commands
        .spawn(SpriteBundle {
            texture: halo_texture_handle,
            visibility: Visibility::Hidden,
            transform: Transform {
                scale: scale_factor,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CardHalo);

    let initial_position = Vec3::new(-520.0, 240.0, 0.0);
    let card_spacing = 25.0;

    for (i, _card) in deck.cards.iter().enumerate() {
        let offset_x = i as f32 * card_spacing;
        let position = initial_position + Vec3::new(offset_x, 0.0, 0.0);

        commands
            .spawn(SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform {
                    translation: position,
                    scale: scale_factor,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(_card.clone());
    }
}
