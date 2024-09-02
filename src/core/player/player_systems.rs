const PLAYER_RADIUS: f32 = 30.0;
const JUMP_FORCE: f32 = 500.0;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::circle(PLAYER_RADIUS),
        GravityScale(10.0),
        LinearVelocity::default(),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: PLAYER_RADIUS,
            })),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

pub fn player_movement_input(
    mut players: Query<&mut LinearVelocity, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut player in players.iter_mut() {
        if keys.just_pressed(KeyCode::Space) {
            info!("Jumping");
            player.y = JUMP_FORCE;
        };
    }
}
