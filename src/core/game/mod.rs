use bevy::prelude::*;

mod bird;
mod collision;
mod pipe;
mod ui;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bird::BirdPlugin,
            pipe::PipePlugin,
            collision::CollisionPlugin,
            ui::UiPlugin,
        ));
    }
}
