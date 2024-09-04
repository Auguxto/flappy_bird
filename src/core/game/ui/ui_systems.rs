use bevy::prelude::*;

use crate::core::{
    assets::FONT_FAMILY,
    game::bird::{bird_events::BirdScoreEvent, bird_resources::BirdResources},
};

use super::ui_components::{
    UIContainer, UIScoreContainer, UIScoreResult, UIScoreResultImage, UIScoreResultImages,
};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut score_images: Vec<UiImage> = Vec::new();

    for _ in 0..2 {
        score_images.push(UiImage::new(asset_server.load("sprites/score/0.png")));
    }

    // Main UI Conatiner
    let ui_container = commands
        .spawn((
            UIContainer,
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Relative,
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    ..default()
                },
                z_index: ZIndex::Global(i32::MAX),
                ..default()
            },
            Name::new("UIContainer"),
        ))
        .id();

    // Score Container - Top Left
    let ui_score_container = commands
        .spawn((
            UIScoreContainer,
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("UIScoreContainer"),
        ))
        .id();

    // Score Text
    let ui_score_result = commands
        .spawn((
            UIScoreResult,
            TextBundle {
                text: Text::from_section(
                    "SCORE: ",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::WHITE,
                        font: asset_server.load(FONT_FAMILY),
                    },
                ),
                ..default()
            },
            Name::new("UIScoreResult"),
        ))
        .id();

    // Score images container
    let ui_score_result_images_container = commands
        .spawn((
            UIScoreResultImages,
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            },
            Name::new("UIScoreResultImages"),
        ))
        .id();

    // Add default images to images container - 3 Images 0 0 0
    for _ in 0..3 {
        let ui_score_zero_value = UiImage::new(asset_server.load("sprites/score/0.png"));

        // Default image component
        let ui_score_result_image = commands
            .spawn((
                UIScoreResultImage,
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        width: Val::Px(24.0),
                        height: Val::Px(36.0),
                        margin: UiRect {
                            left: Val::Px(2.0),
                            right: Val::Px(2.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                        },
                        ..default()
                    },
                    ..default()
                },
                // Inspector
                Name::new("UIScoreResultImage"),
                ui_score_zero_value,
            ))
            .id();

        commands
            .entity(ui_score_result_images_container)
            .add_child(ui_score_result_image);
    }

    commands
        .entity(ui_score_container)
        .push_children(&[ui_score_result, ui_score_result_images_container]);
    commands
        .entity(ui_container)
        .push_children(&[ui_score_container]);
}

pub fn update_ui_score_result(
    mut commands: Commands,
    mut event_reader_bird_score_event: EventReader<BirdScoreEvent>,
    bird_resouces: Res<BirdResources>,
    score_images_containers: Query<Entity, With<UIScoreResultImages>>,
    score_images: Query<Entity, With<UIScoreResultImage>>,
    asset_server: Res<AssetServer>,
) {
    for _ in event_reader_bird_score_event.read() {
        let score_text = format!("{:03}", bird_resouces.score);

        for score_image in &score_images {
            commands.entity(score_image).despawn_recursive();
        }

        for score_image_container in &score_images_containers {
            for letter in score_text.chars() {
                let ui_score_value =
                    UiImage::new(asset_server.load(format!("sprites/score/{letter}.png")));

                let ui_score_result_image = commands
                    .spawn((
                        UIScoreResultImage,
                        NodeBundle {
                            style: Style {
                                width: Val::Px(24.0),
                                height: Val::Px(36.0),
                                margin: UiRect {
                                    left: Val::Px(2.0),
                                    right: Val::Px(2.0),
                                    top: Val::Px(0.0),
                                    bottom: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        // Image
                        ui_score_value,
                        // Inspector
                        Name::new("UIScoreResultImage"),
                    ))
                    .id();

                commands
                    .entity(score_image_container)
                    .add_child(ui_score_result_image);
            }
        }
    }
}
