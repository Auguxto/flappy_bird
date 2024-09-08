use bevy::prelude::*;

use core::CorePlugin;

pub mod config;
mod core;

fn main() {
    App::new().add_plugins(CorePlugin).run();
}
