use bevy::prelude::*;

use crate::{
    random::DiceBox,
    resource::Config,
    component::*
};

pub fn spawn_monster<'a>(
    commands: &'a mut Commands,
    rnd: &mut DiceBox,
    config: &Config,
    position: UVec2
) -> EntityCommands<'a> {
    let n = rnd.roll_dice(1, u8::MAX as i32);
    let position = position.into();

    match n % 2 {
        0 => commands.spawn(GoblinBundle::new(position, config.viewshed_range)),
        1 => commands.spawn(OrcBundle::new(position, config.viewshed_range)),
        _ => unreachable!()
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    rnd: &mut DiceBox,
    area: URect
) {
    let n = rnd.roll_dice(1, 6);

    for _ in 0..n {
        let x = area.min.x + rnd.roll_dice(1, area.width() as i32) as u32;
        let y = area.min.y + rnd.roll_dice(1, area.height() as i32) as u32;

        let position = [x, y].into();

        let bundle = PotionBundle::new(position);
        commands.spawn(bundle);
    }
}
