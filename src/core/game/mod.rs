use bevy::prelude::*;

mod bird;
mod collision;
mod pipe;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collision::CollisionPlugin,
            bird::BirdPlugin,
            pipe::PipePlugin,
        ));
    }
}
