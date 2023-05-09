
use std::fs;
use bevy::prelude::Component;
use toml;

use serde::Deserialize;

#[derive(Debug, Clone, Component, Deserialize)]
pub struct EncounterCard {
    pub name: String,
}


#[derive(Debug, Component, Clone)]
pub struct EncounterDeck {
    pub cards: Vec<EncounterCard>,
}


#[derive(Debug, Deserialize)]
pub struct EncounterCardsList {
    pub encounters: Vec<EncounterCard>,
}

pub fn load_encounter_cards() -> Vec<EncounterCard> {
    let toml_str = fs::read_to_string("assets/toml/encounters.toml")
        .expect("Could not read the encounters.toml file");

    let cards_list: EncounterCardsList = toml::from_str(&toml_str)
        .expect("Could not deserialize the encounters.toml content");

    cards_list.encounters
}