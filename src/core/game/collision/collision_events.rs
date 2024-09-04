use bevy::prelude::*;

#[derive(Event)]
pub struct BirdHitPipeEvent {
    pub bird: Entity,
    pub pipe: Entity,
}

#[derive(Event)]
pub struct BirdEndHitScoreEvent {
    pub bird: Entity,
    pub score_area: Entity,
}
