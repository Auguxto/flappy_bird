use bevy::prelude::*;

pub mod pipe_components;
mod pipe_configs;
mod pipe_resources;
mod pipe_systems;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<pipe_resources::PipeResources>()
            // Tick pipe spawn interval
            .add_systems(Update, pipe_systems::tick_pipe_spawn_interval)
            // Spawn pipes, run after tick_pipe_spawn_interval system
            .add_systems(
                Update,
                pipe_systems::spawn_pipe.after(pipe_systems::tick_pipe_spawn_interval),
            )
            // Run after main systems
            .add_systems(
                Update,
                (pipe_systems::pipe_movement, pipe_systems::pipe_despawn)
                    .after(pipe_systems::spawn_pipe),
            )
            .add_systems(Update, pipe_systems::pipe_hit);
    }
}
