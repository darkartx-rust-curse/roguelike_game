use std::cmp;

use bevy::prelude::{ResMut, ButtonInput, KeyCode, Query, With, IVec2};

use crate::component::{Position, Player};

pub(super) fn player_input(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut player: Query<&mut Position, With<Player>>
) {
    let key = keyboard_input.get_pressed().next().cloned();
    let mut player = player.single_mut().unwrap();

    let mut new_position = IVec2::ZERO;

    if let Some(key) = key {
        log::debug!("Key pressed: {key:?}");

        match key {
            KeyCode::ArrowRight => { new_position.x += 1 }
            KeyCode::ArrowDown => { new_position.y -= 1 }
            KeyCode::ArrowLeft => { new_position.x -= 1 }
            KeyCode::ArrowUp => { new_position.y += 1 }
            _ => {}
        }
    }

    if new_position != IVec2::ZERO {
        try_move_player(player.as_mut(), new_position);
    }
}

fn try_move_player(player: &mut Position, delta: IVec2) {
    player.0.x = cmp::min(79, cmp::max(0, player.0.x + delta.x));
    player.0.y = cmp::min(49, cmp::max(0, player.0.y + delta.y));
}
