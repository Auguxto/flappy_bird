use avian2d::prelude::{Collider, LinearVelocity, LockedAxes, RigidBody, Sensor};
use bevy::color::palettes::css::{GREEN, PURPLE};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;

use rand::{thread_rng, Rng};

use crate::core::game::collision::collision_events::BirdHitPipeEvent;

use super::pipe_components::{Pipe, ScoreArea};
use super::pipe_configs::{PIPE_MOVEMENT_SPEED, SPACE_BETWEEN_PIPES_IN_PERCENT};
use super::pipe_resources::PipeResources;

pub fn tick_pipe_spawn_interval(time: Res<Time>, mut pipes_resources: ResMut<PipeResources>) {
    pipes_resources.spawn_interval.tick(time.delta());
}

pub fn spawn_pipe(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    pipe_resources: Res<PipeResources>,
) {
    if pipe_resources.spawn_interval.finished() {
        let mut rng = thread_rng();

        // Main window
        let window = windows.get_single().unwrap();

        // Window resolutions
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        // Half window resolution
        let window_width_half = window_width / 2.0;
        let window_height_half = window_height / 2.0;

        // Width 1% of window width
        let pipe_width = window_width * 0.08;

        // Spawn pipe X transform position
        let pipe_x_transform = window_width_half + (pipe_width / 2.0);

        // Min is % of window height
        let min_top_pipe_height = window_height * SPACE_BETWEEN_PIPES_IN_PERCENT;

        // Max is half of window height
        let max_top_pipe_height = window_height / 2.0;

        // Random top pipe height
        let top_pipe_height = rng.gen_range(min_top_pipe_height..max_top_pipe_height);
        // Top pipe transform
        let top_pipe_transform = Transform::from_xyz(
            pipe_x_transform,
            window_height_half - (top_pipe_height / 2.0),
            0.0,
        );

        let score_area_transform = Transform::from_xyz(
            pipe_x_transform,
            top_pipe_transform.translation.y - top_pipe_height / 2.0 - min_top_pipe_height / 2.0,
            0.0,
        );

        // Bottom pipe height
        let bottom_pipe_height = window_height - top_pipe_height - min_top_pipe_height;
        // Bottom pipe transform
        let bottom_pipe_transform = Transform::from_xyz(
            pipe_x_transform,
            -(window_height_half - (bottom_pipe_height / 2.0)),
            0.0,
        );

        let pipe_locked_axes = LockedAxes::new().lock_translation_y().lock_rotation();

        commands.spawn((
            ScoreArea,
            // Physics
            RigidBody::Kinematic,
            pipe_locked_axes,
            Collider::rectangle(pipe_width, min_top_pipe_height - 2.0),
            Sensor,
            // Trasnform
            score_area_transform,
            // Inspector
            Name::new("Score Area"),
        ));

        // Top pipe
        let _top_pipe = commands
            .spawn((
                Pipe,
                // Physics
                RigidBody::Dynamic,
                pipe_locked_axes,
                Collider::rectangle(pipe_width, top_pipe_height),
                // Mesh
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(pipe_width, top_pipe_height))),
                    material: materials.add(Color::from(GREEN)),
                    transform: top_pipe_transform,
                    ..default()
                },
                // Inspector
                Name::new("Pipe"),
            ))
            .id();

        // Bottom pipe
        let _bottom_pipe = commands
            .spawn((
                Pipe,
                // Physics
                RigidBody::Dynamic,
                pipe_locked_axes,
                Collider::rectangle(pipe_width, bottom_pipe_height),
                // Mesh
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(pipe_width, bottom_pipe_height))),
                    material: materials.add(Color::from(GREEN)),
                    transform: bottom_pipe_transform,
                    ..default()
                },
                // Inspector
                Name::new("Pipe"),
            ))
            .id();
    }
}

pub fn pipe_movement(mut pipes_linear_velocities: Query<&mut LinearVelocity, With<Pipe>>) {
    for mut pipe_velocity in &mut pipes_linear_velocities {
        pipe_velocity.x = -PIPE_MOVEMENT_SPEED;
    }
}

pub fn score_area_movement(
    mut score_areas_linear_velocities: Query<&mut LinearVelocity, With<ScoreArea>>,
) {
    for mut score_area_velocity in &mut score_areas_linear_velocities {
        score_area_velocity.x = -PIPE_MOVEMENT_SPEED;
    }
}

pub fn pipe_despawn(
    mut commands: Commands,
    pipes: Query<(Entity, &Transform), With<Pipe>>,
    score_areas: Query<(Entity, &Transform), With<ScoreArea>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Main window
    let window = windows.get_single().unwrap();

    // Window resolutions
    let window_width = window.resolution.width();

    // Half window resolution
    let window_width_half = window_width / 2.0;

    // Width 1% of window width
    let pipe_width = window_width * 0.08;

    let max_x_transform = -(window_width_half + (pipe_width / 2.0));

    for (pipe, pipe_transform) in &pipes {
        if pipe_transform.translation.x <= max_x_transform {
            commands.entity(pipe).despawn();
        }
    }

    for (score_area, score_area_transform) in &score_areas {
        if score_area_transform.translation.x <= max_x_transform {
            commands.entity(score_area).despawn();
        }
    }
}

pub fn pipe_hit(
    mut event_bird_hit_pipe: EventReader<BirdHitPipeEvent>,
    mut pipes: Query<(Entity, &mut Handle<ColorMaterial>), With<Pipe>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_bird_hit_pipe.read() {
        let pipe_hited = event.pipe;

        for (pipe, pipe_color) in &mut pipes {
            if pipe_hited == pipe {
                if let Some(material) = materials.get_mut(&*pipe_color) {
                    material.color = Color::from(PURPLE);
                }
            }
        }
    }
}
