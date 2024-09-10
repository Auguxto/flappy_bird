use avian2d::prelude::{Collider, GravityScale, RigidBody};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::bird_components::Bird;

#[derive(Bundle)]
pub struct BirdBundle {
    bird: Bird,
    rigidbody: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
    // mesh: MaterialMesh2dBundle<ColorMaterial>,
    sprite: SpriteBundle,
    name: Name,
}

impl BirdBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &mut ResMut<AssetServer>,
        bird_radius: f32,
    ) -> Self {
        let mesh = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: bird_radius,
            })),
            material: materials.add(Color::WHITE),
            ..default()
        };

        Self {
            // mesh,
            bird: Bird,
            rigidbody: RigidBody::Dynamic,
            collider: Collider::circle(bird_radius),
            gravity_scale: GravityScale(10.0),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/bird/yellowbird-midflap.png"),
                transform: Transform::default(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(34.0 * 2.0, 24.0 * 2.0)),
                    ..default()
                },
                ..default()
            },
            name: Name::new("Bird"),
        }
    }
}
