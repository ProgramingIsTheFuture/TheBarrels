use bevy::prelude::*;

use crate::network;
use crate::player;

use crate::global_state::entities::EntitiesController;
use crate::windows::status::WindowStatus;

pub struct WindowsPlugin {}

impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugins(DefaultPlugins)
            .add_state(WindowStatus::MenuWindow)
            .add_system(handle_window_changer.system())
            .add_system_set(
                SystemSet::on_enter(WindowStatus::MenuWindow)
                    .with_system(handle_menu_window.system()),
            )
            .add_plugin(player::plugin::PlayerPlugin {})
            .add_plugin(network::plugin::NetworkPlugin {})
            /*
            .add_system(handle_close_disconnect.system())
            
            .add_system_set(
                SystemSet::on_enter(WindowStatus::InGameWindow).with_system(setup_camera.system()),
            )*/;
    }
}

fn handle_menu_window(mut commands: Commands, mut entities: ResMut<EntitiesController>) {
    for ent in entities.entities.iter_mut() {
        commands.entity(ent.clone()).despawn();
    }

    entities.entities = vec![];
}

fn handle_window_changer(
    keyboard_input: Res<Input<KeyCode>>,
    mut wind_status: ResMut<State<WindowStatus>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) {
        match wind_status.current() {
            WindowStatus::InGameWindow => wind_status.set(WindowStatus::MenuWindow),
            WindowStatus::MenuWindow => wind_status.set(WindowStatus::InGameWindow),
        };
    }
}
