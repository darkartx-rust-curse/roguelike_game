use bevy::prelude::{Bundle, Component, UVec2};

pub use crate::map::Map;

// Компонент указывающий позицию в мире всего что имеет почицию
// Игрока, Врагов, Тайлы карты
#[derive(Debug, Clone, Copy, Component)]
pub struct Position(pub UVec2);

impl From<UVec2> for Position {
    fn from(value: UVec2) -> Self {
        Self(value)
    }
}

impl From<[u32; 2]> for Position {
    fn from(value: [u32; 2]) -> Self {
        Self(value.into())
    }
}

impl From<(u32, u32)> for Position {
    fn from(value: (u32, u32)) -> Self {
        Self(value.into())
    }
}

// Компонент-флаг игрока
#[derive(Debug, Component)]
pub struct Player;

// Бандл из компонентов игрока
#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Position
}

impl PlayerBundle {
    pub fn new(position: Position) -> Self {
        Self { player: Player, position }
    }
}

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug, Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub position: Position,
    pub left_mover: LeftMover
}

impl EnemyBundle {
    pub fn new(position: Position) -> Self {
        Self { enemy: Enemy, position, left_mover: LeftMover }
    }
}

#[derive(Debug, Component)]
pub struct LeftMover;
