use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use super::bird_components::Bird;
use super::bird_configs::{BIRD_JUMP_FORCE, BIRD_RADIUS};

// Spawning Bird(Sprite, Collider, RigidiBody etc)
pub fn spawn_bird(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let _bird = commands
        .spawn((
            Bird,
            // Physics
            RigidBody::Dynamic,
            Collider::circle(BIRD_RADIUS),
            GravityScale(10.0),
            Restitution::new(0.0),
            // Mesh, Sprite
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: BIRD_RADIUS,
                })),
                material: materials.add(Color::WHITE),
                transform: Transform::default(),
                ..default()
            },
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
