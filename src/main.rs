use bevy::prelude::*;
use std::env;

mod debug;

fn main() {
    let mut debug_mode = false;
    for arg in env::args() {
        if arg == "debug" {
            debug_mode = true;
        }
    }

    let mut app_builder = App::build();

    app_builder.add_plugins(DefaultPlugins);

    if debug_mode {
        app_builder.add_plugin(debug::plugin::DebugPlugin {});
    }

    app_builder.run();
}
