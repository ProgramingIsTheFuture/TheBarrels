use bevy::prelude::*;
use std::env;

mod debug;
mod network;

fn main() {
    let mut debug_mode = false;
    for arg in env::args() {
        if arg == "debug" {
            debug_mode = true;
        }
    }

    let mut app_builder = App::build();

    app_builder.add_plugins(DefaultPlugins);
    app_builder.add_plugin(network::plugin::NetworkPlugin {});
    if debug_mode {
        app_builder.add_plugin(debug::plugin::DebugPlugin {});
    }

    app_builder.run();
}
