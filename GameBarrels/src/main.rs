use bevy::prelude::*;
use std::env;

mod debug;
mod global_state;
mod network;
mod player;
mod windows;

fn main() {
    let mut debug_mode = false;
    for arg in env::args() {
        if arg == "debug" {
            debug_mode = true;
        }
    }

    let mut app_builder = App::build();

    app_builder.insert_resource(global_state::entities::EntitiesController { entities: vec![] });
    app_builder.insert_resource(global_state::entities::StateStruct::default());

    app_builder.add_plugin(windows::plugin::WindowsPlugin {});
    if debug_mode {
        app_builder.add_plugin(debug::plugin::DebugPlugin {});
    }

    /*
    app_builder.add_plugin(network::plugin::NetworkPlugin {});
    app_builder.add_plugin(player::plugin::PlayerPlugin {});
    */

    app_builder.run();
}
