use bevy::prelude::*;
use std::env;

mod debug;
mod network;
mod player;

fn main() {
    let mut debug_mode = false;
    for arg in env::args() {
        if arg == "debug" {
            debug_mode = true;
        }
    }

    let mut app_builder = App::build();

    app_builder.add_plugins(DefaultPlugins);
    app_builder.add_plugin(player::plugin::PlayerPlugin {});
    app_builder.add_plugin(network::plugin::NetworkPlugin {});
    if debug_mode {
        app_builder.add_plugin(debug::plugin::DebugPlugin {});
    }

    app_builder.add_startup_system(setup_camera.system());
    app_builder.add_startup_system(setup.system());

    app_builder.run();
}

fn setup_camera(mut commands: Commands) {
    // Spawn the camera perspective
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(00.0, 00.0, 50.0)
            .looking_at(Vec3::from([0.0, 0.0, 0.0]), Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Insert character sprites
    commands.insert_resource(player::types::PlayerType {
        player: materials.add(asset_server.load("sprites/default.png").into()),
    });
}
