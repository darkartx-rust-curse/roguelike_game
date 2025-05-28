use bevy::prelude::*;

use crate::{
    event::PlayerSpawnedEvent,
    component::*,
    map::{Map, generator::*},
    resource::*
};

use super::resource::*;

pub(super) fn generate_map(mut commands: Commands, mut rnd: ResMut<DiceBox>) {
    // let mut map_generator = NoisyGenerator::new((80, 50).into(), &mut rnd);
    let mut map_generator = DungeonGenerator::new(
        (80, 50).into(),
        30,
        6,
        20,
        rnd.as_mut()
    );
    let map = map_generator.generate();

    commands.spawn(map);
}

pub(super) fn spawn_player(mut commands: Commands, map: Query<&Map>, mut player_swpaned_event: EventWriter<PlayerSpawnedEvent>) {
    let map = map.single().unwrap();

    let entity = commands.spawn(PlayerBundle::new(map.player_spawn_point().into(), 6));
    player_swpaned_event.write(PlayerSpawnedEvent(entity.id()));
}

pub(super) fn turn_delay(
    time: Res<Time>,
    mut turn_timer: ResMut<TurnTimer>,
    mut next_turn_state: ResMut<NextState<TurnState>>
) {
    turn_timer.tick(time.delta());

    if turn_timer.finished() {
        next_turn_state.set(TurnState::PlayerTurn);
        turn_timer.reset();
    }
}

pub(super) fn turn_start(mut next_turn_state: ResMut<NextState<TurnState>>) {
    next_turn_state.set(TurnState::PlayerTurn);
}
