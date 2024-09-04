use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;

use crate::core::game::collision::collision_events::BirdEndHitScoreEvent;

use super::bird_components::Bird;
use super::bird_configs::{BIRD_JUMP_FORCE, BIRD_RADIUS};
use super::bird_events::BirdScoreEvent;
use super::bird_resources::BirdResources;

// Spawning Bird(Sprite, Collider, RigidiBody etc)
pub fn spawn_bird(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single().unwrap();

    let window_with = window.resolution.width();

    let bird_radius = window_with * 0.03;

    let _bird = commands
        .spawn((
            Bird,
            // Physics
            RigidBody::Dynamic,
            Collider::circle(bird_radius),
            GravityScale(10.0),
            Restitution::new(0.0),
            // Mesh, Sprite
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: bird_radius,
                })),
                material: materials.add(Color::WHITE),
                transform: Transform::default(),
                ..default()
            },
            // Inspector
            Name::new("Bird"),
        ))
        .id();
}

// Bird jump system
pub fn bird_jump_input(
    mut birds_query: Query<&mut LinearVelocity, With<Bird>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut linear_velocity in &mut birds_query {
        if keyboard.just_pressed(KeyCode::Space) {
            linear_velocity.y = BIRD_JUMP_FORCE;
        }
    }
}

pub fn update_bird_score(
    mut bird_resources: ResMut<BirdResources>,
    mut event_bird_score: EventWriter<BirdScoreEvent>,
    mut event_bird_end_hit_score_area: EventReader<BirdEndHitScoreEvent>,
) {
    for _event in event_bird_end_hit_score_area.read() {
        let score = &mut bird_resources.score;
        *score += 1;
        event_bird_score.send(BirdScoreEvent);
    }
}
