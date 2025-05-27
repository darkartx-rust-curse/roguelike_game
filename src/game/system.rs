use bevy::prelude::{Commands, Query, With, ResMut};

use crate::{component::*, map_generator::*, resource::*};

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

pub(super) fn spawn_player(mut commands: Commands, map: Query<&Map>) {
    let map = map.single().unwrap();

    commands.spawn(PlayerBundle::new(map.player_spawn_point().into()));
}

pub(super) fn spawn_enemies(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn(EnemyBundle::new((i * 7, 20).into()));
    }
}

pub(super) fn left_walker(entities: Query<&mut Position, With<LeftMover>>) {
    for mut position in entities {
        position.0.x -= 1;
        if position.0.x < 0 { position.0.x = 79; }
    }
}