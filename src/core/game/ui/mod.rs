use bevy::prelude::*;

mod ui_components;
mod ui_systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_systems::spawn_ui)
            .add_systems(Update, ui_systems::update_ui_score_result);
    }
}
