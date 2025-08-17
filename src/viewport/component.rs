use bevy::prelude::*;
use bevy_ascii_terminal::{color::*,Tile};

use crate::{
    component::{Player, Enemy},
    map::MapTile
};

use super::constants;

#[derive(Component)]
pub(super) struct ViewportTerminal;

pub trait ToTile {
    fn to_tile(&self, visible: bool) -> Tile;
}

impl ToTile for Player {
    fn to_tile(&self, _visible: bool) -> Tile {
        Tile {
            glyph: '@',
            fg_color: constants::PLAYER_FG,
            bg_color: constants::PLAYER_BG,
        }
    }
}

impl ToTile for MapTile {
    fn to_tile(&self, visible: bool) -> Tile {
        match self {
            MapTile::Void => Tile::new(' ', BLACK, BLACK),
            MapTile::Floor => {
                if visible {
                    Tile::new('.', constants::MAP_TILE_FLOOR_VISIBLE_FG, constants::MAP_TILE_FLOOR_VISIBLE_BG)
                } else {
                    Tile::new('.', constants::MAP_TILE_FLOOR_FG, constants::MAP_TILE_FLOOR_BG)
                }
            }
            MapTile::Wall => {
                if visible {
                    Tile::new('#', constants::MAP_TILE_WALL_VISIBLE_FG, constants::MAP_TILE_WALL_VISIBLE_BG)
                } else {
                    Tile::new('#', constants::MAP_TILE_WALL_FG, constants::MAP_TILE_WALL_BG)
                }
            }
        }
    }
}

impl ToTile for Enemy {
    fn to_tile(&self, _visible: bool) -> Tile {
        let glyph = self.name().chars().next().unwrap_or('e');

        Tile::new(
            glyph,
            constants::ENEMY_FG,
            constants::ENEMY_BG
        )
    }
}

#[derive(Debug, Component)]
pub(super) struct Cursor {
    pub position: UVec2,
    pub show: bool
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            position: UVec2::default(),
            show: true
        }
    }
}

impl Cursor {
    pub(super) fn hide(&mut self) {
        self.position = UVec2::default();
        self.show = false;
    }

    pub(super) fn show_at(&mut self, position: UVec2) {
        self.position = position;
        self.show = true;
    }
}
