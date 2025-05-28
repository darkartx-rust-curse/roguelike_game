use bevy::prelude::*;

use crate::{
    event::*,
    component::*,
    map::{Map, generator::*},
    resource::*
};

use super::{VIEWSHED_RANGE, resource::*};

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

// Спавн игрока
pub(super) fn spawn_player(
    mut commands: Commands,
    map: Query<&Map>,
    mut player_swpaned_event: EventWriter<PlayerSpawnedEvent>
) {
    let map = match map.single() {
        Ok(map) => map,
        _ => return,
    };

    let entity = commands.spawn(
        PlayerBundle::new(
            map.player_spawn_point().into(),
            VIEWSHED_RANGE,
            map.make_revealed_map()
        )
    );
    player_swpaned_event.write(PlayerSpawnedEvent(entity.id()));
}

// Спавн монстров
pub(super) fn spawn_enemies(
    mut commands: Commands,
    map: Query<&Map>,
    mut enemy_spawned_event: EventWriter<EnemySpawnedEvent>,
    mut rnd: ResMut<DiceBox>
) {
    let map = match map.single() {
        Ok(map) => map,
        _ => return,
    };

    for position in map.monster_spawn_points() {
        let enemy = enemy_randomizer(rnd.as_mut());

        let entity = commands.spawn(
            EnemyBundle::new(enemy, position.into(), VIEWSHED_RANGE)
        );

        enemy_spawned_event.write(EnemySpawnedEvent(entity.id()));
    }
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

fn enemy_randomizer(rnd: &mut DiceBox) -> Enemy {
    let n = rnd.roll_dice(1, u8::MAX as i32);

    match n % 2 {
        0 => Enemy::Goblin,
        1 => Enemy::Orc,
        _ => unreachable!()
    }
}
