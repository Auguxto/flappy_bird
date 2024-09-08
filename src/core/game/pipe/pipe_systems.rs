use avian2d::prelude::*;
use bevy::color::palettes::css::PURPLE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{thread_rng, Rng};

use crate::config::{PIPE_MOVEMENT_SPEED, SPACE_BETWEEN_PIPES_IN_PERCENT};
use crate::core::game::collision::collision_events::BirdHitPipeEvent;

use super::pipe_bundles::{PipeBundle, ScoreAreaBundle};
use super::pipe_components::{Pipe, ScoreArea};
use super::pipe_resources::PipeResources;

// Tick spawn pipe interval
pub fn tick_pipe_spawn_interval(time: Res<Time>, mut pipes_resources: ResMut<PipeResources>) {
    pipes_resources.spawn_interval.tick(time.delta());
}

// Spawning pipes
pub fn spawn_pipe(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    pipe_resources: Res<PipeResources>,
) {
    if pipe_resources.spawn_interval.finished() {
        let mut rng = thread_rng();

        // Primary Window
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

        // Min is % of window height (is also the height of score area)
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

        // Spawning score area
        commands.spawn(ScoreAreaBundle::new(
            pipe_width,
            // Min 5.0 to not overlap pipe collider
            min_top_pipe_height - 5.0,
            score_area_transform,
        ));

        // Spawning top pipe
        commands.spawn(PipeBundle::new(
            &mut meshes,
            &mut materials,
            pipe_width,
            top_pipe_height,
            top_pipe_transform,
        ));

        // Spawning bottom pipe
        commands.spawn(PipeBundle::new(
            &mut meshes,
            &mut materials,
            pipe_width,
            bottom_pipe_height,
            bottom_pipe_transform,
        ));
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

    // Width 8% of window width
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

pub fn despawn_all_pipes(
    mut commands: Commands,
    pipes: Query<Entity, With<Pipe>>,
    score_areas: Query<Entity, With<ScoreArea>>,
) {
    for pipe in &pipes {
        commands.entity(pipe).despawn();
    }

    for score_area in &score_areas {
        commands.entity(score_area).despawn();
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
