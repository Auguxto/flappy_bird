use bevy::prelude::*;

use state_states::ScreenState;
use state_systems::*;

pub mod state_states;
mod state_systems;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ScreenState>()
            .add_systems(Startup, debug_current_gamemode_state);
    }
}
