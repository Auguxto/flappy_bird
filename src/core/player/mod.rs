use bevy::prelude::*;

mod player_systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_systems::spawn_player)
            .add_systems(Update, player_systems::player_movement_input);
    }
}
