use bevy::prelude::States;

pub use crate::random::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    Setup,
    PlayerTurn,
    EndTurn,
}
