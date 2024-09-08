use avian2d::prelude::{Collider, CollisionMargin, LockedAxes, RigidBody, Sensor};
use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::pipe_components::{Pipe, ScoreArea};

// *********** Start Pipe ***********

#[derive(Bundle)]
pub struct PipeBundle {
    name: Name,
    pipe: Pipe,
    collider: Collider,
    rigidbody: RigidBody,
    locked_axes: LockedAxes,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl PipeBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        width: f32,
        height: f32,
        transform: Transform,
    ) -> Self {
        let locked_axes = LockedAxes::new().lock_translation_y().lock_rotation();

        let mesh = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(width, height))),
            material: materials.add(Color::from(GREEN)),
            transform,
            ..default()
        };

        Self {
            mesh,
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
