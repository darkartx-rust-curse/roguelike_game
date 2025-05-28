use bevy::prelude::States;

pub use crate::random::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    Setup,
    PlayerTurn,
    EnemyTurn,
    EndTurn,
}

impl TurnState {
    pub fn next(current: Self) -> Self {
        match current {
            Self::Setup => Self::Setup,
            Self::PlayerTurn => Self::EnemyTurn,
            Self::EnemyTurn => Self::EndTurn,
            Self::EndTurn => Self::EndTurn,
        }
    }
}
