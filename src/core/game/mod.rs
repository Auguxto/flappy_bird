use bevy::prelude::*;

mod bird;
mod collision;
mod pipe;
mod screen_manager;
pub mod state;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::StatesPlugin,
            screen_manager::ScreenManagerPlugin,
            ui::UiPlugin,
            bird::BirdPlugin,
            pipe::PipePlugin,
            collision::CollisionPlugin,
        ));
    }
}
