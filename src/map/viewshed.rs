use bevy::prelude::*;
use bracket_pathfinding::prelude::field_of_view;

use super::{utils::*, Map};

// Видимость сущности
// Для просчета видимости используется bracket-pathfinding
#[derive(Debug, Clone, Component)]
pub struct Viewshed {
    range: u32,
    visible_tiles: Vec<UVec2>,
}

impl Viewshed {
    pub fn new(range: u32) -> Self {
        Self { range, visible_tiles: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.visible_tiles.clear();
    }

    pub fn visible_tiles(&self) -> &[UVec2] {
        &self.visible_tiles
    }

    pub(super) fn recalculate(&mut self, map: &Map, position: UVec2) {
        self.clear();

        let map_rect = URect::from_corners(UVec2::ZERO, map.size());

        let visible_tiles = field_of_view(
            position.to_point(),
            self.range as i32,
            map
        );

        self.visible_tiles = visible_tiles.iter()
            .map(|point| point.to_uvec2())
            .filter(|point| map_rect.contains(*point))
            .collect();
    }
}
