use bevy::prelude::*;

mod camera_systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_systems::setup_camera);
    }
}
