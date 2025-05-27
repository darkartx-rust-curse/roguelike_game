use bevy::prelude::{Component, UVec2};

use crate::renderer::tile;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MapTile {
    Floor = 0,
    Wall = 1,
}

#[derive(Debug, Clone, Component)]
pub struct Map {
    size: UVec2,
    tiles: Vec<MapTile>,
}

impl Map {
    pub fn new(size: UVec2, tiles: Vec<MapTile>) -> Self {
        assert_eq!(tiles.len(), (size.x * size.y) as usize, "Map size not equal tiles size");

        Self { size, tiles }
    }

    pub fn with_size(size: UVec2) -> Self {
        let vec_size = (size.x * size.y) as usize;
        let tiles = vec![MapTile::Floor; vec_size];

        Self { size, tiles }
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
}

fn position_to_index(position: UVec2, size: UVec2) -> usize {
    assert!(position.y <= size.y && position.x <= size.x, "Position out of size bounds");

    (position.y * size.x + position.x) as usize
}