use bevy::prelude::*;

use crate::{
    component::{*, Name},
    map::Pathfinder,
};

pub(super) fn enemy_ai(
    mut map: Query<&mut Map>,
    enemies: Query<(&mut Position, &Enemy, &Name, &Viewshed)>,
    player: Query<&Position, (With<Player>, Without<Enemy>)>
) {
    let mut map = match map.single_mut() {
        Ok(map) => map,
        _ => return,
    };

    for (mut position, _enemy, name, viewshed) in enemies {
        let near_player = player.iter()
            .find(|position| viewshed.visible_tiles().contains(&position.0));

        if let Some(player) = near_player {
            log::debug!("{} see player", name.0);

            if position.0.as_vec2().distance(player.0.as_vec2()) <= 1.5 {
                log::debug!("{} insults player", name.0);
                continue;
            }

            let pathfinder = Pathfinder::new(&map);
            let path_to_player = pathfinder.find_path(position.0, player.0);

            let move_position = match path_to_player {
                Some(path) => path.into_iter().skip(1).next(),
                _ => None
            };

            if let Some(move_position) = move_position {
                map.set_blocked(move_position, true);
                position.0 = move_position;
            }
        }
    }
}
