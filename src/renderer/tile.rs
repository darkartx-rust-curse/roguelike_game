use bevy_ascii_terminal::{color::*,Tile};

use crate::{
    component::{Player},
    map::MapTile
};

use super::color;

pub trait ToTile {
    fn to_tile(&self, visible: bool) -> Tile;
}

impl ToTile for Player {
    fn to_tile(&self, _visible: bool) -> Tile {
        Tile {
            glyph: '@',
            fg_color: color::PLAYER_FG,
            bg_color: color::PLAYER_BG,
        }
    }
}

impl ToTile for MapTile {
    fn to_tile(&self, visible: bool) -> Tile {
        match self {
            MapTile::Void => Tile::new(' ', BLACK, BLACK),
            MapTile::Floor => {
                if visible {
                    Tile::new('.', color::MAP_TILE_FLOOR_VISIBLE_FG, color::MAP_TILE_FLOOR_VISIBLE_BG)
                } else {
                    Tile::new('.', color::MAP_TILE_FLOOR_FG, color::MAP_TILE_FLOOR_BG)
                }
            }
            MapTile::Wall => {
                if visible {
                    Tile::new('#', color::MAP_TILE_WALL_VISIBLE_FG, color::MAP_TILE_WALL_VISIBLE_BG)
                } else {
                    Tile::new('#', color::MAP_TILE_WALL_FG, color::MAP_TILE_WALL_BG)
                }
            }
        }
    }
}
