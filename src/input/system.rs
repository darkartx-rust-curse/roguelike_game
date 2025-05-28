use bevy::prelude::*;

use crate::{
    component::{Position, Player},
    map::{Map, MapTile},
    resource::TurnState,
};

pub(super) fn player_input(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut player: Query<&mut Position, With<Player>>,
    map: Query<&Map>,
    mut next_turn_state: ResMut<NextState<TurnState>>
) {
    let mut player = match player.single_mut() {
        Ok(player) => player,
        _ => return,
    };

    let key = keyboard_input.get_pressed().next().cloned();

    if let Some(key) = key {
        // log::debug!("Key pressed: {key:?}");

        let mut player_delta = IVec2::ZERO;

        match key {
            KeyCode::ArrowRight => { player_delta.x += 1 }
            KeyCode::ArrowDown => { player_delta.y -= 1 }
            KeyCode::ArrowLeft => { player_delta.x -= 1 }
            KeyCode::ArrowUp => { player_delta.y += 1 }
            _ => {}
        }

        if player_delta != IVec2::ZERO {
            let map = map.single().unwrap();

            try_move_player(player.as_mut(), player_delta, map);
        }

        next_turn_state.set(TurnState::EndTurn)
    }
}

fn try_move_player(player: &mut Position, delta: IVec2, map: &Map) {
    let min_point = IVec2::ZERO;
    let max_point = map.size().as_ivec2();

    let new_position = (player.0.as_ivec2() + delta).min(max_point).max(min_point);
    
    match map.tile(new_position.as_uvec2()) {
        MapTile::Floor => { player.0 = new_position.as_uvec2() }
        _ => {}
    }
}
