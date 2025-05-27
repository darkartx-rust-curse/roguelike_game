use bevy::prelude::{UVec2};

use crate::{
    map::{Map, MapTile},
    random::DiceBox
};

use super::Generator;

pub struct NoisyGenerator<'a> {
    size: UVec2,
    rnd: &'a mut DiceBox
}

impl<'a> NoisyGenerator<'a> {
    pub fn new(size: UVec2, rnd: &'a mut DiceBox) -> Self {
        Self { size, rnd }
    }
}

impl Generator for NoisyGenerator<'_> {
    fn generate(&mut self) -> Map {
        let mut map = Map::with_size(self.size);

        for x in 0..map.size().x {
            map.set_tile((x, 0).into(), MapTile::Wall);
            map.set_tile((x, map.size().y - 1).into(), MapTile::Wall);
        }

        for y in 1..(map.size().y - 1) {
            map.set_tile((0, y).into(), MapTile::Wall);
            map.set_tile((map.size().x - 1, y).into(), MapTile::Wall);
        }

        for _i in 0..400 {
            let x = self.rnd.roll_dice(1, (self.size.x - 1) as i32) as u32;
            let y = self.rnd.roll_dice(1, (self.size.y - 1) as i32) as u32;
            map.set_tile((x, y).into(), MapTile::Wall);
        }
        
        map
    }
}