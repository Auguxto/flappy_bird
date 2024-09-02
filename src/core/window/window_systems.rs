use bevy::{core::FrameCount, prelude::*, window::PrimaryWindow};

pub fn make_window_visible(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    frames: Res<FrameCount>,
) {
    // GPU ready to render app
    if frames.0 == 3 {
        windows.single_mut().visible = true;
    }
}
