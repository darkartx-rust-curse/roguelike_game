use bevy::color::LinearRgba;
use bevy_ascii_terminal::color::hex_color;

pub(super) const UI_SIZE: usize = 8;
pub(super) const HEALTHBAR_SIZE: usize = 50;
pub(super) const HEALTHBAR_COLOR: LinearRgba = hex_color(0xff0000);
pub(super) const HEALTHBAR_FULL_GLYPH: char = '▓';
pub(super) const HEALTHBAR_EMPTY_GLYPH: char = '░';
