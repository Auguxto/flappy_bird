use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenState {
    #[default]
    MainMenu,
    InGame,
    Dead,
}
