use bevy::prelude::*;
use collision_events::{BirdEndHitScoreEvent, BirdHitPipeEvent};

pub mod collision_events;
mod collision_systems;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BirdHitPipeEvent>()
            .add_event::<BirdEndHitScoreEvent>()
            .add_systems(Update, collision_systems::bird_and_pipe_collision_detect)
            .add_systems(
                Update,
                collision_systems::bird_and_score_area_end_collision_detect,
            );
    }
}
