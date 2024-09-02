use bevy::prelude::*;

#[derive(Event)]
pub struct BirdHitPipeEvent {
    pub bird: Entity,
    pub pipe: Entity,
}
