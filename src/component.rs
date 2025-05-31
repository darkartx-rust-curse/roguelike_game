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
    pub creature_bundle: CreatureBundle,
    pub player: Player,
    pub revealed_map: RevealedMap,
    pub player_command: PlayerCommand,
}

impl PlayerBundle {
    pub fn new(position: Position, viewshed_range: u32, revealed_map: RevealedMap) -> Self {
        let creature_bundle = CreatureBundle::new(
            "Player".to_string(),
            position,
            viewshed_range,
            30,
            1_000,
            5
        );

        Self {
            creature_bundle,
            player: Player,
            revealed_map,
            player_command: PlayerCommand::default(),
        }
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
    pub creature_bundle: CreatureBundle,
    pub enemy: Enemy,
    pub blocks_tile: BlocksTile,
}

impl EnemyBundle {
    pub fn new(enemy: Enemy, position: Position, viewshed_range: u32) -> Self {
        let creature_bundle = CreatureBundle::new(
            enemy.name(),
            position,
            viewshed_range,
            16,
            1,
            4
        );

        Self {
            creature_bundle,
            enemy,
            blocks_tile: BlocksTile,
        }
    }

    pub fn with_name(mut self, name: Name) -> Self {
        self.creature_bundle.name = name;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component)]
pub enum PlayerCommand {
    #[default]
    None,
    MoveUp,
    MoveRight,
    MoveDown,
    MoveLeft,
    MoveUpRight,
    MoveUpLeft,
    MoveDownRight,
    MoveDownLeft,
}

#[derive(Debug, Component)]
pub struct BlocksTile;

#[derive(Debug, Component)]
pub struct MaxHelth(pub u32);

#[derive(Debug, Component)]
pub struct Health(pub u32);

#[derive(Debug, Component)]
pub struct Defence(pub u32);

#[derive(Debug, Component)]
pub struct Power(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component)]
pub enum CreatureIntention {
    #[default]
    Nothing,
    Move(UVec2),
    Attack(UVec2),
}

impl CreatureIntention {
    pub fn is_move(&self) -> bool {
        match self {
            Self::Move(_) => true,
            _ => false
        }
    }

    pub fn is_attack(&self) -> bool {
        match self {
            Self::Attack(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Bundle)]
pub struct CreatureBundle {
    pub name: Name,
    pub position: Position,
    pub viewshed: Viewshed,
    pub max_health: MaxHelth,
    pub health: Health,
    pub defence: Defence,
    pub power: Power,
    pub creature_intention: CreatureIntention,
    pub damages: Damages,
}

impl CreatureBundle {
    pub fn new(
        name: String,
        position: Position,
        viewshed_range: u32,
        max_health: u32,
        defence: u32,
        power: u32
    ) -> Self {
        let name = Name(name);
        let viewshed = Viewshed::new(viewshed_range);

        Self {
            name,
            position,
            viewshed,
            max_health: MaxHelth(max_health),
            health: Health(max_health),
            defence: Defence(defence),
            power: Power(power),
            creature_intention: CreatureIntention::default(),
            damages: Damages::default(),
        }
    }
}

// Грязный дамаг
#[derive(Debug, Default, Component)]
pub struct Damages(pub Vec<u32>);
