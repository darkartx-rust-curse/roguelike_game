use bevy::prelude::{Bundle, Component, UVec2};

pub use crate::map::{Map, Viewshed, RevealedMap};

// Компонент указывающий позицию в мире всего что имеет почицию
// Игрока, Врагов, Тайлы карты
#[derive(Debug, Clone, Copy, Component)]
pub struct Position(pub UVec2);

impl From<UVec2> for Position {
    fn from(value: UVec2) -> Self {
        Self(value)
    }
}

impl From<&UVec2> for Position {
    fn from(value: &UVec2) -> Self {
        Self(*value)
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

// Компонет имя
#[derive(Debug, Component)]
pub struct Name(pub String);

// Компонент-флаг игрока
#[derive(Debug, Component)]
pub struct Player;

// Бандл из компонентов игрока
#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Position,
    pub viewshed: Viewshed,
    pub revealed_map: RevealedMap
}

impl PlayerBundle {
    pub fn new(position: Position, viewshed_range: u32, revealed_map: RevealedMap) -> Self {
        let viewshed = Viewshed::new(viewshed_range);

        Self { player: Player, position, viewshed, revealed_map }
    }
}

// Компонент врага
#[derive(Debug, Default, Component)]
pub enum Enemy {
    #[default]
    Goblin,
    Orc
}

impl Enemy {
    pub fn name(&self) -> String {
        match self {
            Self::Goblin => "Goblin".to_string(),
            Self::Orc => "Orc".to_string()
        }
    }
}

// Бандл из компонентов игрока
#[derive(Debug, Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub name: Name,
    pub position: Position,
    pub viewshed: Viewshed,
}

impl EnemyBundle {
    pub fn new(enemy: Enemy, position: Position, viewshed_range: u32) -> Self {
        let viewshed = Viewshed::new(viewshed_range);
        let name = Name(enemy.name());

        Self { enemy, position, viewshed, name }
    }

    pub fn with_name(mut self, name: Name) -> Self {
        self.name = name;
        self
    }
}
