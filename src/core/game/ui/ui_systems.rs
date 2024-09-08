use bevy::prelude::*;

use crate::config::FONT_FAMILY;
use crate::core::game::bird::{bird_events::BirdScoreEvent, bird_resources::BirdResources};

use super::bundles::text_bundle::UIText;
use super::screens::commom::containers::UIContainerFlexCenter;
use super::ui_components::*;

pub fn spawn_main_ui(mut commands: Commands) {
    // Main UI Container
    commands.spawn((
        UIContainer,
        NodeBundle {
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..default()
            },
            z_index: ZIndex::Global(i32::MAX),
            ..default()
        },
        Name::new("UIContainer"),
    ));
}

pub fn spawn_dead_screen(
    mut commands: Commands,
    mut bird_resouces: ResMut<BirdResources>,
    mut asset_server: ResMut<AssetServer>,
    ui_containers: Query<Entity, With<UIContainer>>,
) {
    if let Ok(ui_container) = ui_containers.get_single() {
        let dead_screen = commands
            .spawn((UIDead, UIContainerFlexCenter::new("UIDead")))
            .id();
        let score_text = commands
            .spawn(UIText::new(
                format!("Score: {}", bird_resouces.score).as_str(),
                &mut asset_server,
            ))
            .id();
        let restart_text = commands
            .spawn(UIText::new(
                "Pressione 'Espaço' para reiniciar.",
                &mut asset_server,
            ))
            .id();

        bird_resouces.score = 0;

        commands
            .entity(dead_screen)
            .push_children(&[score_text, restart_text]);
        commands.entity(ui_container).push_children(&[dead_screen]);
    }
}

pub fn despawn_dead_screen(mut commands: Commands, dead_screens: Query<Entity, With<UIDead>>) {
    if let Ok(dead_screen) = dead_screens.get_single() {
        commands.entity(dead_screen).despawn_recursive();
    }
}

pub fn spawn_main_menu(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    ui_containers: Query<Entity, With<UIContainer>>,
) {
    if let Ok(ui_container) = ui_containers.get_single() {
        let loading = commands
            .spawn((UIMainMenu, UIContainerFlexCenter::new("UILoadingContainer")))
            .id();

        let text = commands
            .spawn(UIText::new(
                "Pressione 'Espaço' para começar",
                &mut asset_server,
            ))
            .id();

        commands.entity(loading).push_children(&[text]);
        commands.entity(ui_container).push_children(&[loading]);
    }
}

pub fn despawn_main_menu(mut commands: Commands, ui_loadings: Query<Entity, With<UIMainMenu>>) {
    for ui_loading in &ui_loadings {
        commands.entity(ui_loading).despawn_recursive();
    }
}

pub fn spawn_score_ui(
    ui_containers: Query<Entity, With<UIContainer>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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

    for ui_container in &ui_containers {
        commands
            .entity(ui_container)
            .push_children(&[ui_score_container]);
    }
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

pub fn despawn_score_ui(
    mut commands: Commands,
    ui_score_scontainers: Query<Entity, With<UIScoreContainer>>,
) {
    for ui_score_container in &ui_score_scontainers {
        commands.entity(ui_score_container).despawn_recursive();
    }
}
