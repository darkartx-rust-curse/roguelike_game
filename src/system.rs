use bevy::prelude::*;

use crate::{
    event::*,
    component::{*, Name},
    map::{Map, generator::*},
    resource::*
};

pub(super) fn setup_game(mut game_log: ResMut<GameLog>) {
    game_log.clear();
}

pub(super) fn generate_map(mut commands: Commands, mut rnd: ResMut<DiceBox>, config: Res<Config>) {
    // let mut map_generator = NoisyGenerator::new((80, 50).into(), &mut rnd);
    let mut map_generator = DungeonGenerator::new(
        (config.map_width, config.map_height).into(),
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
    mut player_swpaned_event: EventWriter<PlayerSpawnedEvent>,
    config: Res<Config>
) {
    let map = match map.single() {
        Ok(map) => map,
        _ => return,
    };

    let entity = commands.spawn(
        PlayerBundle::new(
            map.player_spawn_point().into(),
            config.viewshed_range,
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
    mut rnd: ResMut<DiceBox>,
    config: Res<Config>
) {
    let map = match map.single() {
        Ok(map) => map,
        _ => return,
    };

    for position in map.monster_spawn_points() {
        let enemy = enemy_randomizer(rnd.as_mut());

        let entity = commands.spawn(
            EnemyBundle::new(enemy, position.into(), config.viewshed_range)
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

pub(super) fn process_player_commands(
    player_commands: Query<(Entity, &mut PlayerCommand, &mut CreatureIntention)>,
    targets: Query<(Entity, &Position), With<Health>>
) {
    use PlayerCommand::*;

    for (entity, mut player_command, mut creature_intention) in player_commands {
        let position = targets.iter().find(|(target_entity, _)| *target_entity == entity);
        let position = match position {
            Some((_, position)) => position,
            _ => continue
        };
        let mut player_delta = IVec2::ZERO;

        match player_command.as_ref() {
            MoveUp | MoveUpLeft | MoveUpRight => { player_delta.y += 1; }
            MoveDown | MoveDownLeft | MoveDownRight => { player_delta.y -= 1; }
            _ => { }
        }

        match player_command.as_ref() {
            MoveRight | MoveDownRight | MoveUpRight => { player_delta.x += 1; }
            MoveLeft | MoveDownLeft | MoveUpLeft => { player_delta.x -= 1; }
            _ => { }
        }

        if player_delta != IVec2::ZERO {
            let target_position = (position.0.as_ivec2() + player_delta).as_uvec2();
            let is_target = targets.iter()
                .any(|(_, position)| position.0 == target_position);

            *creature_intention = if is_target {
                CreatureIntention::Attack(target_position)
            } else {
                CreatureIntention::Move(target_position)
            };
        }

        *player_command = PlayerCommand::default();
    }
}

pub(super) fn movement_system(
    mut map: Query<&mut Map>,
    mut creature_intentions: Query<(
        &mut Position, &mut CreatureIntention
    )>
) {
    let mut map = match map.single_mut() {
        Ok(map) => map,
        _ => return
    };

    let map_rect = map.rect();

    let movement_intentions = creature_intentions.iter_mut()
        .filter(|(_, intention)| intention.is_move());

    for (mut position, mut intention) in movement_intentions {
        let target_position = match *intention {
            CreatureIntention::Move(target_position) => target_position,
            _ => unreachable!()
        };

        *intention = CreatureIntention::default();
        
        let target_position = target_position.min(map_rect.max).max(map_rect.min);

        if !map.blocked(target_position) {
            map.set_blocked(target_position, true);
            position.0 = target_position;
        }
    }
}

pub(super) fn combat_system(
    mut attackers: Query<(&mut CreatureIntention, &Position, &Power, Option<&Name>)>,
    mut targets: Query<(&Position, &mut Damages)>
) {
    let attackers = attackers.iter_mut()
        .filter(|intension| intension.0.is_attack());

    for (
        mut attack_intension,
        position,
        power,
        attacker_name
    ) in attackers {
        let attack_position = match *attack_intension {
            CreatureIntention::Attack(attack_position) => attack_position,
            _ => unreachable!()
        };

        *attack_intension = CreatureIntention::Nothing;

        let attack_position_vec = attack_position.as_vec2();
        let position_vec = position.0.as_vec2();

        if position_vec.distance(attack_position_vec) > 1.5 {
            continue;
        }

        for (target_position, mut damages) in targets.iter_mut() {
            if target_position.0 != attack_position {
                continue;
            }

            let damage = power.0;

            damages.add_damage(
                Damage::new(
                    damage, 
                    DamageCause::Creature(
                        attacker_name.map(|name| name.0.clone())
                    )
                )
            );
        }
    }
}

pub(super) fn damage_system(
    mut commands: Commands,
    creatures: Query<(Entity, &mut Health, &mut Damages, Option<&Defence>, Option<&Name>)>,
    mut game_log: ResMut<GameLog>
) {
    for (entity, mut health, mut damages, defence, name) in creatures {
        if damages.items().is_empty() {
            continue;
        }

        let name = name.map(|name| name.0.clone()).unwrap_or("Unknown".to_string());
        let defence = defence.map(|defence| defence.0).unwrap_or(0);

        let mut total_damage = 0u32;

        for damage in damages.items() {
            if damage.value <= defence {
                continue;
            }

            let damage_dealer = match damage.cause.clone() {
                DamageCause::Creature(name) => name.clone()
            };
            let dealed_damage = damage.value - defence;

            game_log.add_entry(
                GameLogEntry::Damage(
                    damage_dealer.unwrap_or("Unknown".to_string()),
                    name.clone(),
                    dealed_damage
                )
            );

            total_damage += dealed_damage;
        }
        damages.clear();

        if total_damage > health.0 {
            health.0 = 0;
        } else {
            health.0 -= total_damage;
        }

        if health.0 == 0 {
            game_log.add_entry(GameLogEntry::Dead(name));
            commands.entity(entity).despawn();
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
