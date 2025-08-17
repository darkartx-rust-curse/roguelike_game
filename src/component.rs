use bevy::prelude::*;

pub use crate::map::{Map, Viewshed, RevealedMap};

use crate::constants::*;

// Компонент указывающий позицию в мире всего что имеет почицию
// Игрока, Врагов, Тайлы карты
#[derive(Debug, Clone, Copy, Component, PartialEq)]
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
            PLAYER_MAX_HP,
            PLAYER_DEFENCE,
            PLAYER_POWER
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
pub struct Enemy;

// Бандл из компонентов игрока
#[derive(Debug, Bundle)]
pub struct EnemyBundle {
    pub creature_bundle: CreatureBundle,
    pub enemy: Enemy,
    pub blocks_tile: BlocksTile,
}

impl EnemyBundle {
    pub fn new(
        name: String,
        position: Position,
        viewshed_range: u32,
        max_health: u32,
        defence: u32,
        power: u32
    ) -> Self {
        let creature_bundle = CreatureBundle::new(
            name,
            position,
            viewshed_range,
            max_health,
            defence,
            power
        );

        Self {
            creature_bundle,
            enemy: Enemy,
            blocks_tile: BlocksTile,
        }
    }

    pub fn with_name(mut self, name: Name) -> Self {
        self.creature_bundle.name = name;
        self
    }
}

// Orc enemy
#[derive(Debug, Bundle)]
pub struct OrcBundle {
    pub enemy_bundle: EnemyBundle
}

impl OrcBundle {
    pub fn new(position: Position, viewshed_range: u32) -> Self {
        let enemy_bundle = EnemyBundle::new(
            "Orc".to_string(),
            position,
            viewshed_range,
            ORC_MAX_HP,
            ORC_DEFENCE,
            ORC_POWER
        );

        Self {
            enemy_bundle
        }
    }
}

// Goblin enemy
#[derive(Debug, Bundle)]
pub struct GoblinBundle {
    pub enemy_bundle: EnemyBundle
}

impl GoblinBundle {
    pub fn new(position: Position, viewshed_range: u32) -> Self {
        let enemy_bundle = EnemyBundle::new(
            "Goblin".to_string(),
            position,
            viewshed_range,
            GOBLIN_MAX_HP,
            GOBLIN_DEFENCE,
            GOBLIN_POWER
        );

        Self {
            enemy_bundle
        }
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
    GrabItem,
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
pub struct Damages(Vec<Damage>);

impl Damages {
    pub fn items(&self) -> &[Damage] {
        &self.0
    }

    pub fn add_damage(&mut self, damage: Damage) {
        self.0.push(damage);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Debug, Clone)]
pub struct Damage {
    pub value: u32,
    pub cause: DamageCause,
}

impl Damage {
    pub fn new(value: u32, cause: DamageCause) -> Self {
        Self {
            value,
            cause,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DamageCause {
    Creature(Option<String>)
}

#[derive(Debug, Component, Default)]
pub struct Item;

#[derive(Debug, Component)]
pub struct Potion {
    pub heal_amount: i32,
}

impl Potion {
    pub fn new(heal_amount: i32) -> Self {
        Self {
            heal_amount
        }
    }
}

#[derive(Debug, Bundle)]
pub struct PotionBundle {
    pub name: Name,
    pub position: Position,
    pub potion: Potion,
    pub item: Item,
}

impl PotionBundle {
    pub fn new(position: Position) -> Self {
        let name = Name("Health Potion".to_string());
        let potion = Potion::new(30);

        Self {
            name,
            position,
            potion,
            item: Item,
        }
    }
}

#[derive(Debug, Component)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Debug, Component)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}
