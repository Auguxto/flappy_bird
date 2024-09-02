use bevy::prelude::*;

use core::CorePlugin;

mod core;

fn main() {
    App::new().add_plugins(CorePlugin).run();
}
