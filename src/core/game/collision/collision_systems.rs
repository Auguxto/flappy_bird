use avian2d::prelude::*;
use bevy::prelude::*;

use crate::core::game::pipe::pipe_components::Pipe;
use crate::core::game::{bird::bird_components::Bird, pipe::pipe_components::ScoreArea};

use super::collision_events::{BirdEndHitScoreEvent, BirdHitPipeEvent};

// Detecting when bird hits a pipe
pub fn bird_and_pipe_collision_detect(
    mut event_bird_and_hit_pipe: EventWriter<BirdHitPipeEvent>,
    birds_collisions: Query<(Entity, &CollidingEntities), With<Bird>>,
    pipes_collisions: Query<Entity, With<Pipe>>,
) {
    // Check all birds (usually there is only one bird)
    for (bird, bird_colliding_entities) in &birds_collisions {
        // Check all pipes collisions and check if bird hits one of them
        for pipe in &pipes_collisions {
            if bird_colliding_entities.contains(&pipe) {
                // Send event BirdHitPipe with bird and pipe entity
                event_bird_and_hit_pipe.send(BirdHitPipeEvent { bird, pipe });
            }
        }
    }
}

// Detecting when bird hitting end a score area
pub fn bird_and_score_area_end_collision_detect(
    mut event_bird_and_score_area_end: EventWriter<BirdEndHitScoreEvent>,
    mut event_bird_end_hit_score_reader: EventReader<CollisionEnded>,
    birds: Query<Entity, With<Bird>>,
    score_areas: Query<Entity, With<ScoreArea>>,
) {
    // Checking all collisions ended
    for CollisionEnded(mut entity1, mut entity2) in event_bird_end_hit_score_reader.read() {
        // If entity1 is a bird and entity2 is a score area
        if birds.contains(entity1) && score_areas.contains(entity2) {
            // Sending BirdEndHitScoreEvent event
            event_bird_and_score_area_end.send(BirdEndHitScoreEvent {
                bird: entity1,
                score_area: entity2,
            });
        }
        // Else entity1 is a score area and entity2 is a bird
        else if score_areas.contains(entity1) && birds.contains(entity2) {
            // Sending BirdEndHitScoreEvent event
            event_bird_and_score_area_end.send(BirdEndHitScoreEvent {
                bird: entity2,
                score_area: entity1,
            });
        };
    }
}
