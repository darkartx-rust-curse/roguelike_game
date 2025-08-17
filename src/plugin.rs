use bevy::prelude::{*, Plugin as BevyPlugin};

use crate::{event::*, inventory_system::*, resource::*, system::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        let config = Config::default();

        app
            .insert_resource(DiceBox::default())
            .insert_resource(config)
            .init_resource::<GameLog>()
            .init_state::<TurnState>()
            .add_event::<PlayerSpawnedEvent>()
            .add_event::<EnemySpawnedEvent>()
            .add_systems(Startup, (
                setup_game,
                generate_map,
                (spawn_player, spawn_enemies, spawn_items, start_turn)
            ).chain())
            .add_systems(Update, (turn_system, item_collection_system))
            .add_systems(OnExit(TurnState::PlayerTurn), process_player_commands)
            .add_systems(OnEnter(TurnState::EndTurn), (movement_system, combat_system, damage_system).chain())
        ;
    }
}