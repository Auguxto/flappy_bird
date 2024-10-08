use bevy::prelude::*;

use crate::config::PIPE_SPAWN_INTERVAL;

#[derive(Resource)]
pub struct PipeResources {
    pub spawn_interval: Timer,
}

impl Default for PipeResources {
    fn default() -> Self {
        Self {
            spawn_interval: Timer::from_seconds(PIPE_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}
