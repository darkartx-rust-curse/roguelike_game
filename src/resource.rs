use bevy::prelude::States;

pub use crate::random::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    Setup,
    StartTurn,
    PlayerTurn,
    EnemyTurn,
    EndTurn,
}

impl TurnState {
    pub fn next(&self) -> Self {
        match self {
            Self::Setup => Self::Setup,
            Self::StartTurn => Self::PlayerTurn,
            Self::PlayerTurn => Self::EnemyTurn,
            Self::EnemyTurn => Self::EndTurn,
            Self::EndTurn => Self::EndTurn,
        }
    }
}
