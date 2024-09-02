use avian2d::prelude::*;
use bevy::prelude::*;

use crate::core::game::bird::bird_components::Bird;
use crate::core::game::pipe::pipe_components::Pipe;

use super::collision_events::BirdHitPipeEvent;

pub fn collision_detect(
    mut event_bird_hit_pipe: EventWriter<BirdHitPipeEvent>,
    birds_collisions: Query<(Entity, &CollidingEntities), With<Bird>>,
    pipes_collisions: Query<(Entity, &CollidingEntities), With<Pipe>>,
) {
    // Check all birds (usually there is only one bird)
    for (bird, bird_colliding_entities) in &birds_collisions {
        // Check all pipes collisions and check if bird hits one of them
        for (pipe, _pipe_colliding_entities) in &pipes_collisions {
            if bird_colliding_entities.contains(&pipe) {
                // Send event BirdHitPipe with bird and pipe entity
                event_bird_hit_pipe.send(BirdHitPipeEvent { bird, pipe });
            }
        }
    }
}
