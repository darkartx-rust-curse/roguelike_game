use bevy::color::LinearRgba;
use bevy_ascii_terminal::color::{hex_color, BLACK, RED, MAGENTA, WHITE, GRAY};

pub(super) const MAP_TILE_WALL_FG: LinearRgba = hex_color(0x101010);
pub(super) const MAP_TILE_WALL_BG: LinearRgba = BLACK;
pub(super) const MAP_TILE_WALL_VISIBLE_FG: LinearRgba = hex_color(0x8a610a);
pub(super) const MAP_TILE_WALL_VISIBLE_BG: LinearRgba = BLACK;
pub(super) const MAP_TILE_FLOOR_FG: LinearRgba = hex_color(0x050505);
pub(super) const MAP_TILE_FLOOR_BG: LinearRgba = BLACK;
pub(super) const MAP_TILE_FLOOR_VISIBLE_FG: LinearRgba = hex_color(0x606050);
pub(super) const MAP_TILE_FLOOR_VISIBLE_BG: LinearRgba = BLACK;
pub(super) const PLAYER_FG: LinearRgba = hex_color(0xffff00);
pub(super) const PLAYER_BG: LinearRgba = BLACK;
pub(super) const ENEMY_FG: LinearRgba = RED;
pub(super) const ENEMY_BG: LinearRgba = BLACK;
pub(super) const CURSOR_COLOR: LinearRgba = MAGENTA;
pub(super) const TOOLTIP_FG: LinearRgba = WHITE;
pub(super) const TOOLTIP_BG: LinearRgba = GRAY;
