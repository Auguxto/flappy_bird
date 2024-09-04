use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;

mod assets;
mod camera;
mod game;
mod overlay;
mod window;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .build()
                .disable::<WindowPlugin>()
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Physics Plugins
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .insert_resource(Gravity(Vec2::NEG_Y * 100.0))
        .add_plugins(window::WindowPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(game::GamePlugin);
        // .add_plugins(WorldInspectorPlugin::new());
    }
}
