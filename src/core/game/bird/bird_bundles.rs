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
    pub fn new(asset_server: &mut ResMut<AssetServer>) -> Self {
        Self {
            // mesh,
            bird: Bird,
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ellipse(34.0, 24.0),
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
