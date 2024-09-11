use bevy::{prelude::*, window::PrimaryWindow};

use super::bg_components::*;

pub fn setup_bases(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = windows.get_single() {
        let height = window.resolution.height();
        let width = window.resolution.width();

        let half_height = height / 2.0;
        let half_width = width / 2.0;

        let base_width: f32 = 336.0;
        let base_height: f32 = 112.0;

        let startup_bases_count = (width / base_width).ceil() as u32 * 2;

        info!("Bases to print {startup_bases_count}");

        for index in 0..startup_bases_count {
            commands.spawn((
                Base,
                SpriteBundle {
                    texture: asset_server.load("sprites/background/base.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(base_width, base_height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        -half_width + (base_width / 2.0) + index as f32 * base_width,
                        -half_height,
                        1.0,
                    ),
                    ..default()
                },
                Name::new("Base Background"),
            ));
        }
    }
}

pub fn spawn_bases(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    bases: Query<(Entity, &Transform), With<Base>>,
) {
    if let Ok(window) = windows.get_single() {
        let height = window.resolution.height();
        let width = window.resolution.width();

        let half_height = height / 2.0;
        let half_width = width / 2.0;

        let base_width: f32 = 336.0;
        let base_height: f32 = 112.0;

        let bases_total = bases.iter().collect::<Vec<(Entity, &Transform)>>().len() as u32;

        warn!("Total bases: {bases_total}");

        for (base, base_transform) in &bases {
            if base_transform.translation.x <= -half_width - (base_width / 2.0) {
                commands.entity(base).despawn();
                commands.spawn((
                    Base,
                    SpriteBundle {
                        texture: asset_server.load("sprites/background/base.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(base_width, base_height)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            base_transform.translation.x + (bases_total as f32 * base_width),
                            -half_height,
                            1.0,
                        ),
                        ..default()
                    },
                    Name::new("Base Background"),
                ));
            }
        }
    }
}

pub fn move_bases(time: Res<Time>, mut bases: Query<&mut Transform, With<Base>>) {
    for mut base_transform in &mut bases {
        base_transform.translation.x -= 200.0 * time.delta_seconds();
    }
}

pub fn despawn_bases(mut commands: Commands, bases: Query<Entity, With<Base>>) {
    for base in &bases {
        commands.entity(base).despawn();
    }
}
