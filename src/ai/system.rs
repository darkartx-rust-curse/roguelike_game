use bevy::prelude::*;

use crate::{
    component::{*, Name},
    map::Pathfinder,
};

pub(super) fn enemy_ai(
    map: Query<&Map>,
    enemies: Query<(&Position, &mut CreatureIntention, &Name, &Viewshed), With<Enemy>>,
    player: Query<&Position, (With<Player>, Without<Enemy>)>
) {
    let map = match map.single() {
        Ok(map) => map,
        _ => return,
    };

    for (position, mut intension, name, viewshed) in enemies {
        let near_player = player.iter()
            .find(|position| viewshed.visible_tiles().contains(&position.0));

        let current_position = position.0.as_vec2();

        if let Some(player) = near_player {
            log::debug!("{} see player", name.0);

            let player_position = player.0.as_vec2();

            if current_position.distance(player_position) <= 1.5 {
                *intension = CreatureIntention::Attack(player.0)
            } else {
                let pathfinder = Pathfinder::new(&map);
                let path_to_player = pathfinder
                    .find_path(position.0, player.0);

                let target_position = match path_to_player {
                    Some(path) => path.into_iter().skip(1).next(),
                    _ => None
                };

                if let Some(move_position) = target_position {
                    *intension = CreatureIntention::Move(move_position);
                }
            }
        }
    }
}
