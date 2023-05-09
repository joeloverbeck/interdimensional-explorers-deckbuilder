use crate::{
    card_halo::{CardHalo, HALO_TEXTURE_PATH},
    deck::{EncounterCard, EncounterDeck},
};
use bevy::prelude::*;

pub fn card_selection_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut Visibility),
        (Changed<Interaction>, With<CardHalo>),
    >,
    encounter_card_query: Query<&EncounterCard>,
    mut deck_query: Query<(&mut EncounterDeck, &Children)>,
) {
    // This is just an example to select the first card.
    for (entity, interaction, mut visible) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let selected_card = encounter_card_query.get(entity).unwrap();
                // Execute your desired logic for card selection
                // For example: move it to the center and show the halo

                for (mut deck, children) in deck_query.iter_mut() {
                    if let Some(card_index) = deck
                        .cards
                        .iter()
                        .position(|card| card.name == selected_card.name)
                    {
                        commands
                            .entity(children[card_index])
                            .insert(CardHalo)
                            .insert(SpriteBundle {
                                texture: asset_server.load(HALO_TEXTURE_PATH),
                                visibility: Visibility::Visible,
                                ..Default::default()
                            });

                        deck.cards.remove(card_index);
                        break;
                    }
                }
            }
            _ => {
                // If the card is not selected, hide the halo
                *visible = Visibility::Hidden
            }
        }
    }
}
