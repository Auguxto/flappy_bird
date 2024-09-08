use bevy::prelude::*;

use crate::core::game::state::state_states::ScreenState;

pub fn main_menu_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen_state: ResMut<NextState<ScreenState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_screen_state.set(ScreenState::InGame);
    }
}

pub fn dead_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen_state: ResMut<NextState<ScreenState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_screen_state.set(ScreenState::InGame);
    }
}
