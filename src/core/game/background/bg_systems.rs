use bevy::{prelude::*, window::PrimaryWindow};

use super::bg_components::*;

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = windows.get_single() {
        let height = window.resolution.height();
        let width = window.resolution.width();

        let half_height = height / 2.0;
        let half_width = width / 2.0;

        let sprite_width: f32 = 336.0;
        let background_sprite_height: f32 = 228.0;
        let ground_sprite_height: f32 = 112.0;

        let initial_backgrounds_count = (width / sprite_width).ceil() as u32 * 2;

        for index in 0..initial_backgrounds_count {
            // Background
            commands.spawn((
                Background,
                SpriteBundle {
                    texture: asset_server.load("sprites/background/background-day.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(sprite_width, background_sprite_height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        -half_width + (sprite_width / 2.0) + index as f32 * sprite_width,
                        -half_height
                            + (background_sprite_height / 2.0)
                            + (ground_sprite_height / 2.0),
                        -1.0,
                    ),
                    ..default()
                },
                Name::new("Background"),
            ));

            // Ground
            commands.spawn((
                Ground,
                SpriteBundle {
                    texture: asset_server.load("sprites/background/base.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(sprite_width, ground_sprite_height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        -half_width + (sprite_width / 2.0) + index as f32 * sprite_width,
                        -half_height,
                        1.0,
                    ),
                    ..default()
                },
                Name::new("Ground"),
            ));
        }
    }
}

pub fn spawning_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    grounds: Query<(Entity, &Transform), With<Ground>>,
) {
    if let Ok(window) = windows.get_single() {
        let height = window.resolution.height();
        let width = window.resolution.width();

        let half_height = height / 2.0;
        let half_width = width / 2.0;

        let ground_total_count = grounds.iter().collect::<Vec<(Entity, &Transform)>>().len() as u32;

        let sprite_width: f32 = 336.0;
        let sprite_height: f32 = 112.0;

        for (ground, ground_transform) in &grounds {
            if ground_transform.translation.x <= -half_width - (sprite_width / 2.0) {
                commands.entity(ground).despawn();
                commands.spawn((
                    Ground,
                    SpriteBundle {
                        texture: asset_server.load("sprites/background/base.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(sprite_width, sprite_height)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            ground_transform.translation.x
                                + (ground_total_count as f32 * sprite_width)
                                - 2.0,
                            -half_height,
                            1.0,
                        ),
                        ..default()
                    },
                    Name::new("Ground"),
                ));
            }
        }
    }
}

pub fn spawning_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    backgrounds: Query<(Entity, &Transform), With<Background>>,
) {
    if let Ok(window) = windows.get_single() {
        let height = window.resolution.height();
        let width = window.resolution.width();

        let half_height = height / 2.0;
        let half_width = width / 2.0;

        let background_total_count = backgrounds
            .iter()
            .collect::<Vec<(Entity, &Transform)>>()
            .len() as u32;

        let sprite_width: f32 = 336.0;
        let sprite_height: f32 = 228.0;

        let ground_sprite_height: f32 = 112.0;

        for (background, background_transform) in &backgrounds {
            if background_transform.translation.x <= -half_width - (sprite_width / 2.0) {
                commands.entity(background).despawn();
                commands.spawn((
                    Background,
                    SpriteBundle {
                        texture: asset_server.load("sprites/background/background-day.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(sprite_width, sprite_height)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            background_transform.translation.x
                                + (background_total_count as f32 * sprite_width)
                                - 2.0,
                            -half_height + (sprite_height / 2.0) + (ground_sprite_height / 2.0),
                            -1.0,
                        ),
                        ..default()
                    },
                    Name::new("Background"),
                ));
            }
        }
    }
}

pub fn move_ground(time: Res<Time>, mut grounds: Query<&mut Transform, With<Ground>>) {
    for mut ground_transform in &mut grounds {
        ground_transform.translation.x -= 200.0 * time.delta_seconds();
    }
}

pub fn move_background(time: Res<Time>, mut backgrounds: Query<&mut Transform, With<Background>>) {
    for mut background_transform in &mut backgrounds {
        background_transform.translation.x -= 150.0 * time.delta_seconds();
    }
}

pub fn despawn_ground(mut commands: Commands, grounds: Query<Entity, With<Ground>>) {
    for ground in &grounds {
        commands.entity(ground).despawn();
    }
}

pub fn despawn_background(mut commands: Commands, backgrounds: Query<Entity, With<Background>>) {
    for background in &backgrounds {
        commands.entity(background).despawn();
    }
}
