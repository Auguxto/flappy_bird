use avian2d::prelude::{Collider, LockedAxes, RigidBody, Sensor};
use bevy::prelude::*;

use super::pipe_components::{Pipe, ScoreArea};

// *********** Start Pipe ***********

#[derive(Bundle)]
pub struct PipeBundle {
    name: Name,
    pipe: Pipe,
    collider: Collider,
    rigidbody: RigidBody,
    locked_axes: LockedAxes,
    sprite: SpriteBundle,
}

impl PipeBundle {
    pub fn new(
        asset_server: &mut ResMut<AssetServer>,
        width: f32,
        height: f32,
        transform: Transform,
        flip_y: bool,
    ) -> Self {
        let locked_axes = LockedAxes::new().lock_translation_y().lock_rotation();

        let sprite = SpriteBundle {
            texture: asset_server.load("sprites/pipe/pipe-green.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                flip_y,
                ..default()
            },
            transform,
            ..default()
        };

        Self {
            sprite,
            pipe: Pipe,
            locked_axes,
            name: Name::new("Pipe"),
            rigidbody: RigidBody::Dynamic,
            collider: Collider::rectangle(width, height),
        }
    }
}
// *********** Finish Pipe ***********

// *********** Start Score Area ***********

#[derive(Bundle)]
pub struct ScoreAreaBundle {
    name: Name,
    sensor: Sensor,
    collider: Collider,
    rigidbody: RigidBody,
    transform: Transform,
    score_area: ScoreArea,
    locked_axes: LockedAxes,
}

impl ScoreAreaBundle {
    pub fn new(width: f32, height: f32, transform: Transform) -> Self {
        let locked_axes = LockedAxes::new().lock_translation_y().lock_rotation();

        Self {
            transform,
            locked_axes,
            sensor: Sensor,
            score_area: ScoreArea,
            name: Name::new("Score Area"),
            rigidbody: RigidBody::Kinematic,
            collider: Collider::rectangle(width, height),
        }
    }
}

// *********** Finish Score Area ***********
