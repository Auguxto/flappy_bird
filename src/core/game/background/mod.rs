use bevy::prelude::*;

use bg_systems::*;

use super::state::state_states::ScreenState;

mod bg_components;
mod bg_systems;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        // Spawn bases on enter in game state
        app.add_systems(OnEnter(ScreenState::InGame), setup_bases)
            // Move and spawn new bases only in game state
            .add_systems(
                Update,
                (spawn_bases, move_bases).run_if(in_state(ScreenState::InGame)),
            )
            // Remove bases on enter in dead state
            .add_systems(OnEnter(ScreenState::Dead), despawn_bases);
    }
}
