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
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    name: Name,
}

impl BirdBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        bird_radius: f32,
    ) -> Self {
        let mesh = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: bird_radius,
            })),
            material: materials.add(Color::WHITE),
            transform: Transform::default(),
            ..default()
        };

        Self {
            mesh,
            bird: Bird,
            rigidbody: RigidBody::Dynamic,
            collider: Collider::circle(bird_radius),
            gravity_scale: GravityScale(10.0),
            name: Name::new("Bird"),
        }
    }
}
