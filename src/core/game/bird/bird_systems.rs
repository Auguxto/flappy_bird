use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::game::collision::collision_events::BirdEndHitScoreEvent;

use super::bird_bundles::BirdBundle;
use super::bird_components::Bird;
use super::bird_configs::BIRD_JUMP_FORCE;
use super::bird_events::BirdScoreEvent;
use super::bird_resources::BirdResources;

// Spawning Bird
pub fn spawn_bird(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single().unwrap();

    let bird_radius = window.resolution.width() * 0.025;

    commands.spawn(BirdBundle::new(&mut meshes, &mut materials, bird_radius));
}

// Bird jump when pressing Space
pub fn bird_jump_input(
    mut birds_linear_velocities: Query<&mut LinearVelocity, With<Bird>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut bird_linear_velocitiy in &mut birds_linear_velocities {
        if keyboard.just_pressed(KeyCode::Space) {
            bird_linear_velocitiy.y = BIRD_JUMP_FORCE;
        }
    }
}

/**
 * Updating bird score when receive a event BirdEndHitScoreEvent, and sending a
 * event BirdScoreEvent
 */
pub fn update_bird_score(
    mut bird_resources: ResMut<BirdResources>,
    mut event_bird_score: EventWriter<BirdScoreEvent>,
    mut event_bird_end_hit_score_area: EventReader<BirdEndHitScoreEvent>,
) {
    for _ in event_bird_end_hit_score_area.read() {
        let score = &mut bird_resources.score;
        *score += 1;
        event_bird_score.send(BirdScoreEvent);
    }
}
