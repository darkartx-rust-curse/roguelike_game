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