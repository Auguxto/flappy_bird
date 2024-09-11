use bevy::prelude::*;

mod background;
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
            ui::UiPlugin,
            bird::BirdPlugin,
            pipe::PipePlugin,
            state::StatesPlugin,
            collision::CollisionPlugin,
            background::BackgroundPlugin,
            screen_manager::ScreenManagerPlugin,
        ));
    }
}
