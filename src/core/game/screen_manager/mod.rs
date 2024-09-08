use bevy::prelude::*;

use sm_systems::*;

use super::state::state_states::ScreenState;

mod sm_systems;

pub struct ScreenManagerPlugin;

impl Plugin for ScreenManagerPlugin {
    fn build(&self, app: &mut App) {
        // Main menu systems
        app.add_systems(
            Update,
            main_menu_keyboard_input.run_if(in_state(ScreenState::MainMenu)),
        )
        // Dead screen systems
        .add_systems(
            Update,
            dead_keyboard_input.run_if(in_state(ScreenState::Dead)),
        );
    }
}
