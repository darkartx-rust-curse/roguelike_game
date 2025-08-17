use std::cmp;

use bevy::prelude::{URect, UVec2};

use crate::{
    map::{Map, MapTile},
    random::DiceBox
};

use super::Generator;

// Генератор карты подземелья
pub struct DungeonGenerator<'a> {
    size: UVec2,
    max_rooms: u32,
    min_room_size: u32,
    max_room_size: u32,
    rnd: &'a mut DiceBox,
}

impl<'a> DungeonGenerator<'a> {
    pub fn new(
        size: UVec2,
        max_rooms: u32,
        min_room_size: u32,
        max_room_size: u32,
        rnd: &'a mut DiceBox
    ) -> Self {
        Self { size, max_rooms, min_room_size, max_room_size, rnd }
    }
}

impl Generator for DungeonGenerator<'_> {
    fn generate(&mut self) -> Map {
        let mut map = Map::with_tile(self.size, MapTile::Wall);

        let mut rooms = Vec::<URect>::new();

        for _ in 0..self.max_rooms {
            let width = self.rnd.range(self.min_room_size, self.max_room_size);
            let height = self.rnd.range(self.min_room_size, self.max_room_size);
            let x = self.rnd.roll_dice(1, (self.size.x - width - 1) as i32) - 1;
            let y = self.rnd.roll_dice(1, (self.size.y - height - 1) as i32) - 1;
            let x = x as u32;
            let y = y as u32;

            let room = URect::new(x, y, x + width, y + height);
            if rooms.iter().any(|other_room| !room.intersect(*other_room).is_empty()) {
                continue;
            }

            apply_room_to_map(&mut map, room);

            if !rooms.is_empty() {
                let (new_x, new_y) = room.center().into();
                let (prev_x, prev_y) = rooms.last().unwrap().center().into();

                if self.rnd.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(room);
        }

        map.set_player_spawn_point(rooms.first().unwrap().center());

        for room in rooms.iter().skip(1) {
            map.monster_spawn_points_mut().push(room.center());
        }

        for room in rooms {
            let area = room.inflate(-1);
            map.areas_mut().push(area);
        }

        map
    }
}

fn apply_room_to_map(map: &mut Map, room: URect) {
    for y in room.min.y + 1..room.max.y {
        for x in room.min.x + 1..room.max.x {
            map.set_tile((x, y).into(), MapTile::Floor);
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Map, x1: u32, x2: u32, y: u32) {
    for x in cmp::min(x1, x2) ..= cmp::max(x1, x2) {
        map.set_tile((x, y).into(), MapTile::Floor);
    }
}

fn apply_vertical_tunnel(map: &mut Map, y1: u32, y2: u32, x: u32) {
    for y in cmp::min(y1, y2) ..= cmp::max(y1, y2) {
        map.set_tile((x, y).into(), MapTile::Floor);
    }
}
