use bevy::prelude::UVec2;
use bracket_pathfinding::prelude::Point;

pub(super) trait ToPoint {
    fn to_point(&self) -> Point;
}

impl ToPoint for UVec2 {
    fn to_point(&self) -> Point {
        Point { x: self.x as i32, y: self.y as i32 }
    }
}

pub(super) trait ToUVec2 {
    fn to_uvec2(&self) -> UVec2;
}

impl ToUVec2 for Point {
    fn to_uvec2(&self) -> UVec2 {
        UVec2 { x: self.x as u32, y: self.y as u32 }
    }
}

pub fn position_to_index(position: UVec2, size: UVec2) -> usize {
    assert!(position.y <= size.y && position.x <= size.x, "Position out of size bounds");

    (position.y * size.x + position.x) as usize
}

pub fn index_to_position(idx: usize, size: UVec2) -> UVec2 {
    assert!(idx < (size.x * size.y) as usize, "Index out of size bounds");

    let x = idx as u32 % size.x;
    let y = idx as u32 / size.x;

    (x, y).into()
}
