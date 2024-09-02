use bevy::prelude::*;
use collision_events::BirdHitPipeEvent;

pub mod collision_events;
mod collision_systems;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BirdHitPipeEvent>()
            .add_systems(Update, collision_systems::collision_detect);
    }
}
