use bevy::prelude::*;

use crate::{
    component::{Name, *},
    resource::{GameLog, GameLogEntry}
};

pub(super) fn item_collection_system(
    mut commands: Commands,
    player_entity: Query<Entity, With<Player>>,
    wants_to_pickup_items: Query<(Entity, &WantsToPickupItem)>,
    names: Query<(Entity, &Name), With<Item>>,
    mut game_log: ResMut<GameLog>,
) {
    let player_entity = player_entity.single().ok();

    for (entity, wtp_item) in wants_to_pickup_items {
        commands.entity(wtp_item.item)
            .remove::<Position>()
            .insert(InBackpack { owner: wtp_item.collected_by })
        ;

        if player_entity.is_some() && wtp_item.collected_by == player_entity.unwrap() {
            let name = names.iter()
                .find(|(entity, _)| *entity == wtp_item.item)
                .map(|(_, name)| name.0.clone())
            ;

            let name = match name {
                Some(name) => name,
                _ => "Undefined".to_string()
            };

            game_log.add_entry(
                GameLogEntry::PickupItem(name)
            );
        }

        commands.entity(entity).despawn();
    }
    
}