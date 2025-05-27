use bevy_ascii_terminal::{
    color::{BLACK, GREEN, WHITE, GRAY},
    Glyph,
    Tile
};

use crate::{
    component::{Player},
    map::MapTile
};

pub trait ToTile {
    fn to_tile(&self, visible: bool) -> Tile;
}

impl ToTile for Player {
    fn to_tile(&self, _visible: bool) -> Tile {
        Tile {
            glyph: '@',
            fg_color: GREEN,
            bg_color: BLACK
        }
    }
}

impl ToTile for MapTile {
    fn to_tile(&self, visible: bool) -> Tile {
        let glyph = match self {
            MapTile::Void => '.',
            MapTile::Floor => ' ',
            MapTile::Wall => '#',
        };

        let fg_color = if visible {
            GREEN
        } else {
            GRAY
        };

        let bg_color = BLACK;

        Tile {
            glyph,
            fg_color,
            bg_color
        }
    }
}
