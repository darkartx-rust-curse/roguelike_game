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

pub(super) fn turn_system(
    time: Res<Time>,
    mut turn_timer: ResMut<TurnTimer>,
    turn_state: Res<State<TurnState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>
) {
    turn_timer.tick(time.delta());
    let turn_state = *turn_state.get();

    let next_turn = match turn_state {
        TurnState::Setup => None,
        TurnState::StartTurn => {
            turn_timer.reset();
            Some(turn_state.next())
        }
        TurnState::PlayerTurn => {
            if turn_timer.finished() {
                turn_timer.reset();
                Some(turn_state.next())
            } else {
                None
            }
        },
        TurnState::EnemyTurn => Some(turn_state.next()),
        TurnState::EndTurn => Some(TurnState::StartTurn),
    };

    if let Some(next_turn) = next_turn {
        next_turn_state.set(next_turn);
    }
}

pub(super) fn start_turn(mut next_turn_state: ResMut<NextState<TurnState>>) {
    next_turn_state.set(TurnState::StartTurn);
}

pub(super) fn player_movement(map: Query<&Map>, players: Query<(&mut Position, &mut PlayerCommand)>) {
    let map = match map.single() {
        Ok(map) => map,
        _ => return
    };

    use PlayerCommand::*;

    for (mut position, mut command) in players {
        let mut player_delta = IVec2::ZERO;

        match command.as_ref() {
            MoveUp | MoveUpLeft | MoveUpRight => { player_delta.y += 1; }
            MoveDown | MoveDownLeft | MoveDownRight => { player_delta.y -= 1; }
            _ => { }
        }

        match command.as_ref() {
            MoveRight | MoveDownRight | MoveUpRight => { player_delta.x += 1; }
            MoveLeft | MoveDownLeft | MoveUpLeft => { player_delta.x -= 1; }
            _ => { }
        }

        if player_delta != IVec2::ZERO {
            *command = PlayerCommand::default();

            let min_point = IVec2::ZERO;
            let max_point = map.size().as_ivec2();

            let new_position = (position.0.as_ivec2() + player_delta)
                .min(max_point)
                .max(min_point)
                .as_uvec2();

            if !map.blocked(new_position) {
                position.0 = new_position;
            }
        }
    }
}

fn enemy_randomizer(rnd: &mut DiceBox) -> Enemy {
    let n = rnd.roll_dice(1, u8::MAX as i32);

    match n % 2 {
        0 => Enemy::Goblin,
        1 => Enemy::Orc,
        _ => unreachable!()
    }
}
