use bevy::prelude::*;

use ui_systems::*;

use super::state::state_states::ScreenState;

mod bundles;
mod screens;
mod ui_components;
mod ui_systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Spawn main ui
        app.add_systems(Startup, spawn_main_ui)
            .add_systems(Startup, spawn_main_menu.after(spawn_main_ui))
            // Spawn score when enter in screen state in game, and remove loading ui
            .add_systems(
                OnEnter(ScreenState::InGame),
                (despawn_dead_screen, despawn_main_menu, spawn_score_ui),
            )
            // Remove score when enter in screen state dead
            .add_systems(
                OnEnter(ScreenState::Dead),
                (despawn_score_ui, spawn_dead_screen),
            )
            // Remove score when enter in screen state loading
            .add_systems(OnEnter(ScreenState::MainMenu), despawn_score_ui)
            // Update score only when is in screen state in game
            .add_systems(
                Update,
                update_ui_score_result.run_if(in_state(ScreenState::InGame)),
            );
    }
}
