use std::vec;

use bevy::prelude::{UVec2};
use bracket_pathfinding::prelude::*;

use super::{utils::*, Map};

#[derive(Debug, Clone)]
pub struct NavigationPath(pub Vec<UVec2>);

impl IntoIterator for NavigationPath {
    type Item = UVec2;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl NavigationPath {
    pub fn new(steps: Vec<UVec2>) -> Self {
        Self(steps)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pathfinder<'a>(&'a Map);

impl<'a> Pathfinder<'a> {
    pub fn new(map: &'a Map) -> Self {
        Self(map)
    }
}

impl Pathfinder<'_> {
    pub fn find_path(&self, position1: UVec2, position2: UVec2) -> Option<NavigationPath> {
        let map_rect = self.0.rect();
        let map_size = map_rect.max;

        if !map_rect.contains(position1) || !map_rect.contains(position2) {
            return None;
        }

        let start = position_to_index(position1, map_size);
        let end = position_to_index(position2, map_size);

        let path = a_star_search(start, end, self.0);

        if !path.success {
            return None;
        }

        let steps = path.steps.iter()
            .map(|idx| index_to_position(*idx, map_size)).collect();

        Some(NavigationPath::new(steps))
    }
}
