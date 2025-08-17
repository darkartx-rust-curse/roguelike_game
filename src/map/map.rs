use bevy::prelude::{Component, IVec2, UVec2, URect};
use bracket_algorithm_traits::prelude::*;
use bracket_pathfinding::prelude::*;

use super::utils::*;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MapTile {
    Void = 0,
    Floor = 1,
    Wall = 2,
}

#[derive(Debug, Clone, Component)]
pub struct Map {
    size: UVec2,
    tiles: Vec<MapTile>,
    player_spawn_point: UVec2,
    monster_spawn_points: Vec<UVec2>,
    areas: Vec<URect>,
    blocking: Vec<bool>
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        match self.tiles.get(idx) {
            None => true,
            Some(MapTile::Wall | MapTile::Void) => true,
            _ => false
        }
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();

        let idx_position = index_to_position(idx, self.size).as_ivec2();

        let positions = [
            idx_position.with_x(idx_position.x - 1),
            idx_position.with_x(idx_position.x + 1),
            idx_position.with_y(idx_position.y - 1),
            idx_position.with_y(idx_position.y + 1),
            idx_position.with_x(idx_position.x - 1).with_y(idx_position.y - 1),
            idx_position.with_x(idx_position.x - 1).with_y(idx_position.y + 1),
            idx_position.with_x(idx_position.x + 1).with_y(idx_position.y - 1),
            idx_position.with_x(idx_position.x + 1).with_y(idx_position.y + 1)
        ];

        for position in positions {
            if self.is_exit_valid(position) {
                exits.push((position_to_index(position.as_uvec2(), self.size), 1.0));
            }
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let position1 = index_to_position(idx1, self.size);
        let position2 = index_to_position(idx2, self.size);

        DistanceAlg::Pythagoras.distance2d(position1.to_point(), position2.to_point())
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        self.size.to_point()
    }
}

impl Map {
    pub fn new(size: UVec2, tiles: Vec<MapTile>) -> Self {
        assert_eq!(tiles.len(), (size.x * size.y) as usize, "Map size not equal tiles size");

        let blocking = vec![false; tiles.len()];
        let areas = vec![];

        Self {
            size,
            tiles,
            player_spawn_point: UVec2::ZERO,
            monster_spawn_points: Vec::new(),
            areas,
            blocking
        }
    }

    pub fn with_tile(size: UVec2, tile: MapTile) -> Self {
        let vec_size = (size.x * size.y) as usize;
        let tiles = vec![tile; vec_size];

        Self::new(size, tiles)
    }

    pub fn tiles(&self) -> &[MapTile] {
        &self.tiles
    }

    pub fn blocking(&self) -> &[bool] {
        &self.blocking
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn rect(&self) -> URect {
        URect::from_corners(UVec2::ZERO, self.size)
    }

    pub fn tile(&self, position: UVec2) -> MapTile {
        let index = position_to_index(position, self.size);

        match self.tiles.get(index) {
            Some(tile) => *tile,
            _ => MapTile::Void
        }
    }

    pub fn set_tile(&mut self, position: UVec2, value: MapTile) {
        let index = position_to_index(position, self.size);

        match self.tiles.get_mut(index) {
            Some(tile) => { *tile = value; }
            _ => {}
        };
    }

    pub fn blocked(&self, position: UVec2) -> bool {
        let index = position_to_index(position, self.size);
        
        match self.blocking.get(index) {
            Some(blocked) => *blocked,
            _ => true
        }
    }

    pub fn set_blocked(&mut self, position: UVec2, value: bool) {
        let index = position_to_index(position, self.size);

        match self.blocking.get_mut(index) {
            Some(blocked) => { *blocked = value; }
            _ => {}
        };
    }

    pub fn player_spawn_point(&self) -> UVec2 {
        self.player_spawn_point
    }

    pub fn set_player_spawn_point(&mut self, player_spawn_point: UVec2) {
        self.player_spawn_point = player_spawn_point;
    }

    pub fn monster_spawn_points(&self) -> &[UVec2] {
        &self.monster_spawn_points
    }

    pub fn monster_spawn_points_mut(&mut self) -> &mut Vec<UVec2> {
        &mut self.monster_spawn_points
    }

    pub fn areas(&self) -> &[URect] {
        &self.areas
    }

    pub fn areas_mut(&mut self) -> &mut Vec<URect> {
        &mut self.areas
    }

    pub fn make_revealed_map(&self) -> RevealedMap {
        RevealedMap::new(self.size)
    }

    pub(super) fn clear_blocking(&mut self) {
        self.blocking.fill(false);
    }

    pub(super) fn populate_blocking(&mut self) {
        self.clear_blocking();

        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocking[i] = match tile {
                MapTile::Floor => false,
                _ => true,
            }
        }
    }

    fn is_exit_valid(&self, position: IVec2) -> bool {
        if position.x < 0 || position.y < 0 || !self.rect().contains(position.as_uvec2()) {
            return false;
        }

        let idx = position_to_index(position.as_uvec2(), self.size);
        !self.blocking.get(idx).copied().unwrap_or(true)
    }
}

#[derive(Debug, Clone, Component)]
pub struct RevealedMap {
    size: UVec2,
    tiles: Vec<MapTile>,
}

impl RevealedMap {
    pub fn new(size: UVec2) -> Self {
        let tiles = vec![MapTile::Void; (size.x * size.y) as usize];

        Self { size, tiles }
    }

    pub fn tile(&self, position: UVec2) -> MapTile {
        *self.tiles.get(position_to_index(position, self.size)).unwrap()
    }

    pub(super) fn set_tile(&mut self, position: UVec2, tile: MapTile) {
        let index = position_to_index(position, self.size);

        *self.tiles.get_mut(index).unwrap() = tile
    }
}
