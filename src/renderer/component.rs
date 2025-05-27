use bevy_ascii_terminal::{color::{BLACK, RED, WHITE}, Glyph, Tile};

use crate::component::{Player, Enemy};

pub trait ToTile {
    fn to_tile(&self) -> Tile;
}

impl ToTile for Player {
    fn to_tile(&self) -> Tile {
        Tile {
            glyph: '@',
            fg_color: WHITE,
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
