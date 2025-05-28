use std::time;

use bevy::prelude::{Timer, TimerMode, Resource};

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
