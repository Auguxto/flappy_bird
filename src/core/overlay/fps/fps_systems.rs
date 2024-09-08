use bevy::prelude::*;

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use crate::config::FONT_FAMILY;

#[derive(Component)]
pub struct UIFpsContainer;

#[derive(Component)]
pub struct UIFpsText;

pub fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = commands
        .spawn((
            UIFpsContainer,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    let text_fps = commands
        .spawn((
            UIFpsText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 32.0,
                            color: Color::WHITE,
                            font: asset_server.load(FONT_FAMILY),
                        },
                    },
                    TextSection {
                        value: "N/A".into(),
                        style: TextStyle {
                            font_size: 32.0,
                            color: Color::WHITE,
                            font: asset_server.load(FONT_FAMILY),
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<UIFpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections[1].value = format!("{value:>4.0}");

            text.sections[1].style.color = if value >= 120.0 {
                Color::srgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                Color::srgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                Color::srgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                Color::srgb(1.0, 0.0, 0.0)
            }
        } else {
            text.sections[1].value = "N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

/// Toggle the FPS counter when pressing F12
pub fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<UIFpsContainer>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

pub fn despawn_fps_conuter(
    mut commands: Commands,
    fps_counters: Query<Entity, With<UIFpsContainer>>,
) {
    for fps_counter in &fps_counters {
        commands.entity(fps_counter).despawn_recursive();
    }
}
