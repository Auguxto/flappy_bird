use bevy::prelude::*;

mod bird_bundles;
pub mod bird_components;
mod bird_configs;
pub mod bird_events;
pub mod bird_resources;
mod bird_systems;
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<bird_resources::BirdResources>()
            .add_event::<bird_events::BirdScoreEvent>()
            .add_systems(Startup, bird_systems::spawn_bird)
            .add_systems(Update, bird_systems::bird_jump_input)
            .add_systems(Update, bird_systems::update_bird_score);
    }
}
