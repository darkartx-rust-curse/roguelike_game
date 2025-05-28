use bevy::prelude::*;

use crate::{
    component::{*, Name},
    resource::TurnState
};

pub(super) fn enemy_ai(
    enemies: Query<(&Enemy, &Name, &Position, &Viewshed)>,
    player: Query<&Position, With<Player>>,
    turn_state: Res<State<TurnState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>
) {
    let player: Vec<UVec2> = player.iter().map(|position| position.0).collect();

    for (_enemy, name, _position, viewshed) in enemies {
        let see_player = player.iter()
            .any(|player_position| viewshed.visible_tiles().contains(&player_position));

        if see_player {
            log::debug!("{} see player", name.0);
        }
    }

    next_turn_state.set(TurnState::next(**turn_state));
}
