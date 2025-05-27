use bevy::prelude::{Component, UVec2};
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
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        match self.tiles.get(idx) {
            None => true,
            Some(MapTile::Wall | MapTile::Void) => true,
            _ => false
        }
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

        Self { size, tiles, player_spawn_point: UVec2::ZERO }
    }

    pub fn with_tile(size: UVec2, tile: MapTile) -> Self {
        let vec_size = (size.x * size.y) as usize;
        let tiles = vec![tile; vec_size];

        Self::new(size, tiles)
    }

    pub fn tiles(&self) -> &[MapTile] {
        &self.tiles
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn tile(&self, position: UVec2) -> MapTile {
        *self.tiles.get(position_to_index(position, self.size)).unwrap()
    }

    pub fn set_tile(&mut self, position: UVec2, tile: MapTile) {
        let index = position_to_index(position, self.size);

        *self.tiles.get_mut(index).unwrap() = tile
    }

    pub fn player_spawn_point(&self) -> UVec2 {
        self.player_spawn_point
    }

    pub fn set_player_spawn_point(&mut self, player_spawn_point: UVec2) {
        self.player_spawn_point = player_spawn_point;
    }

    pub fn make_revealed_map(&self) -> RevealedMap {
        RevealedMap::new(self.size)
    }
}

#[derive(Debug, Clone, Component)]
pub struct RevealedMap {
    size: UVec2,
    tiles: Vec<MapTile>,
}

impl RevealedMap {
    pub fn new(size: UVec2) -> Self {
        let tiles = vec![MapTile::Floor; (size.x * size.y) as usize];

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

fn position_to_index(position: UVec2, size: UVec2) -> usize {
    assert!(position.y <= size.y && position.x <= size.x, "Position out of size bounds");

    (position.y * size.x + position.x) as usize
}
