use bevy::prelude::*;

use crate::{
    component::{*, Name},
    resource::TurnState,
    map::Pathfinder,
};

pub(super) fn enemy_ai(
    map: Query<&Map>,
    enemies: Query<(&mut Position, &Enemy, &Name, &Viewshed)>,
    player: Query<&Position, (With<Player>, Without<Enemy>)>,
    turn_state: Res<State<TurnState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>
) {
    let map = match map.single() {
        Ok(map) => map,
        _ => return,
    };

    let pathfinder = Pathfinder::new(map);

    for (mut position, _enemy, name, viewshed) in enemies {
        let near_player = player.iter()
            .find(|position| viewshed.visible_tiles().contains(&position.0));

        if let Some(player) = near_player {
            log::debug!("{} see player", name.0);

            let path_to_player = pathfinder.find_path(position.0, player.0);

            let move_position = match path_to_player {
                Some(path) => path.into_iter().skip(1).next(),
                _ => None
            };

            if let Some(move_position) = move_position {
                position.0 = move_position;
            }
        }
    }

    next_turn_state.set(TurnState::next(**turn_state));
}
