use bevy::prelude::*;

pub mod bird_components;
mod bird_configs;
mod bird_systems;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, bird_systems::spawn_bird)
            .add_systems(Update, bird_systems::bird_jump_input);
    }
}
