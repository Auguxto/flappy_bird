use bevy::prelude::*;

use bird_events::*;
use bird_resources::*;
use bird_systems::*;

use super::state::state_states::ScreenState;

mod bird_bundles;
pub mod bird_components;
pub mod bird_events;
pub mod bird_resources;
mod bird_systems;
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BirdResources>()
            .add_event::<BirdScoreEvent>()
            // Run only on enter in game state
            .add_systems(OnEnter(ScreenState::InGame), spawn_bird)
            .add_systems(OnEnter(ScreenState::Dead), despawn_bird)
            .add_systems(
                Update,
                (
                    bird_jump_input,
                    update_bird_score,
                    read_bird_collision_events,
                )
                    // Run if is in game state
                    .run_if(in_state(ScreenState::InGame)),
            );
    }
}
