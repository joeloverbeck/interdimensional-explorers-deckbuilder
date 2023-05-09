
use crate::{deck::EncounterCard};
use bevy::prelude::*;

pub fn move_to_center_system(
    mut query: Query<(&mut Transform, &EncounterCard)>,
) {
    for (mut transform, _card_halo) in query.iter_mut() {
        let target_position = Vec3::new(0.0, 0.0, 1.0);

        let interpolation_factor = 0.01;

        let delta_position = (target_position - transform.translation) * interpolation_factor;
        transform.translation += delta_position;

        if delta_position.length() < 0.001 {
            transform.translation = target_position;
        }
    }
}
