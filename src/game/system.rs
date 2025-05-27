use bevy::prelude::{Commands, Query, With};

use crate::component::*;

pub(super) fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::new((40, 40).into()));
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