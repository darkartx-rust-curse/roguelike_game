use bevy_ascii_terminal::{color::{BLACK, GREEN, RED, WHITE}, Glyph, Tile};

use crate::{
    component::{Player, Enemy},
    map::MapTile
};

pub trait ToTile {
    fn to_tile(&self) -> Tile;
}

impl ToTile for Player {
    fn to_tile(&self) -> Tile {
        Tile {
            glyph: '@',
            fg_color: GREEN,
            bg_color: BLACK
        }
    }
}

impl ToTile for Enemy {
    fn to_tile(&self) -> Tile {
        Tile {
            glyph: Glyph::SmilingFace.into(),
            fg_color: RED,
            bg_color: BLACK
        }
    }
}

impl ToTile for MapTile {
    fn to_tile(&self) -> Tile {
        let glyph = match self {
            MapTile::Void => '.',
            MapTile::Floor => ' ',
            MapTile::Wall => '#',
        };

        Tile {
            glyph,
            fg_color: WHITE,
            bg_color: BLACK
        }
    }
}
