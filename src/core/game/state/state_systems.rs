use bevy::prelude::*;

use super::state_states::ScreenState;

pub fn debug_current_gamemode_state(state: Res<State<ScreenState>>) {
    info!("Current state: {:?}", state.get());
}
