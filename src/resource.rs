use std::fmt;

use bevy::prelude::*;

pub use crate::random::*;

#[derive(Debug, Clone, Resource)]
pub struct Config {
    pub map_width: u32,
    pub map_height: u32,
    pub viewshed_range: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            map_width: 80,
            map_height: 42,
            viewshed_range: 8,
        }
    }
}

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

#[derive(Debug, Default, Resource)]
pub struct GameLog {
    entries: Vec<GameLogEntry>
}

impl GameLog {
    pub fn entries(&self) -> &[GameLogEntry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: GameLogEntry) {
        self.entries.push(entry);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[derive(Debug)]
pub enum GameLogEntry {
    Damage(/* from */String, /* to */String, /* value */u32),
    Dead(String),
}

impl fmt::Display for GameLogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Damage(from, to, value) => {
                write!(f, "{from} hits {to}, for {value} hp.")
            }
            Self::Dead(who) => {
                write!(f, "{who} dead.")
            }
        }
    }
}

