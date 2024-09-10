use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::config::{BIRD_JUMP_FORCE, BIRD_RADIUS};
use crate::core::game::collision::collision_events::{BirdEndHitScoreEvent, BirdHitPipeEvent};
use crate::core::game::state::state_states::ScreenState;

use super::bird_bundles::BirdBundle;
use super::bird_components::Bird;
use super::bird_events::BirdScoreEvent;
use super::bird_resources::BirdResources;

// Spawning Bird
pub fn spawn_bird(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: ResMut<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single().unwrap();

    let bird_radius = window.resolution.width() * BIRD_RADIUS;

    commands.spawn(BirdBundle::new(
        &mut meshes,
        &mut materials,
        &mut asset_server,
        bird_radius,
    ));
}

// Update bird sprite using based on linear velocity
pub fn update_bird_sprite(
    asset_server: Res<AssetServer>,
    mut birds: Query<(&LinearVelocity, &mut Handle<Image>), With<Bird>>,
) {
    for (velocity, mut texture) in &mut birds {
        if velocity.y < 0.0 {
            *texture = asset_server.load("sprites/bird/yellowbird-downflap.png");
        } else if velocity.y >= 0.0 && velocity.y < 100.0 {
            *texture = asset_server.load("sprites/bird/yellowbird-midflap.png");
        } else if velocity.y >= 100.0 {
            *texture = asset_server.load("sprites/bird/yellowbird-upflap.png");
        }
    }
}

// Update bird angle using based on linear velocity
pub fn update_bird_angle(
    asset_server: Res<AssetServer>,
    mut birds: Query<(&LinearVelocity, &mut Transform), With<Bird>>,
) {
    for (velocity, mut trasnform) in &mut birds {
        let angle = velocity.y.remap(1000.0, -1000.0, 0.5, -0.5);
        trasnform.rotation.z = angle;
    }
}

// Despawn bird
pub fn despawn_bird(mut commands: Commands, birds: Query<Entity, With<Bird>>) {
    for bird in &birds {
        commands.entity(bird).despawn();
    }
}

// Bird jump when pressing Space
pub fn bird_jump_input(
    mut birds_linear_velocities: Query<&mut LinearVelocity, With<Bird>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    for mut bird_linear_velocitiy in &mut birds_linear_velocities {
        if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
            bird_linear_velocitiy.y = BIRD_JUMP_FORCE;
        }
    }
}

// Check if bird is going out of screen
pub fn check_bird_out_screen(
    mut next_screen_state: ResMut<NextState<ScreenState>>,
    birds_trasnform: Query<&Transform, With<Bird>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = windows.get_single() {
        if let Ok(bird_transform) = birds_trasnform.get_single() {
            let half_window_height =
                window.resolution.height() / 2.0 - (window.resolution.width() * BIRD_RADIUS);

            if bird_transform.translation.y >= half_window_height
                || bird_transform.translation.y <= -half_window_height
            {
                next_screen_state.set(ScreenState::Dead);
            }
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

pub fn read_bird_collision_events(
    mut bird_hit_pipe_event_reader: EventReader<BirdHitPipeEvent>,
    mut next_screen_state: ResMut<NextState<ScreenState>>,
) {
    // Changing to screen state dead
    for _ in bird_hit_pipe_event_reader.read() {
        next_screen_state.set(ScreenState::Dead);
    }
}
