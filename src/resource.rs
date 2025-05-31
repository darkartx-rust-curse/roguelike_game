use std::{time, fmt};

use bevy::prelude::*;

pub use crate::random::*;

#[derive(Debug, Clone, Resource)]
pub struct Config {
    pub map_width: u32,
    pub map_height: u32,
    pub turn_time_secs: f32,
    pub viewshed_range: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            map_width: 80,
            map_height: 42,
            turn_time_secs: 0.1,
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

#[derive(Debug, Clone, Resource)]
pub(super) struct TurnTimer(Timer);

impl TurnTimer {
    pub fn new(turn_max_time: f32) -> Self {
        let timer = Timer::from_seconds(turn_max_time, TimerMode::Once);

        Self(timer)
    }

    pub fn tick(&mut self, delta: time::Duration) {
        self.0.tick(delta);
    }

    pub fn finished(&self) -> bool {
        self.0.finished()
    }

    pub fn reset(&mut self) {
        self.0.reset();
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

