use bevy::prelude::*;

use pipe_systems::*;

use super::state::state_states::ScreenState;

mod pipe_bundles;
pub mod pipe_components;
mod pipe_resources;
mod pipe_systems;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<pipe_resources::PipeResources>()
            .add_systems(
                Update,
                (
                    tick_pipe_spawn_interval,
                    spawn_pipe,
                    pipe_movement,
                    pipe_despawn,
                    pipe_hit,
                    score_area_movement,
                )
                    .run_if(in_state(ScreenState::InGame)),
            )
            .add_systems(OnEnter(ScreenState::MainMenu), despawn_all_pipes)
            .add_systems(OnEnter(ScreenState::Dead), despawn_all_pipes);
    }
}
