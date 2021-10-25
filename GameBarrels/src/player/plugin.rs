use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    render::{camera::Camera, render_graph::base::camera::CAMERA_3D},
};

use crate::player::types::{Player, PlayerType};

pub struct PlayerInfo;

pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("Spawn player", SystemStage::single(setup_player.system()))
            .add_system(move_champ.system());
    }
}

fn setup_player(mut commands: Commands, player_type: Res<PlayerType>) {
    commands.insert_resource(Player::default());

    commands
        .spawn_bundle(SpriteBundle {
            material: player_type.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 3.),
                scale: Vec3::new(0.001, 0.001, 0.001),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.05, true))
        .insert(PlayerInfo);
}

// This func receives the speed base on the character number
fn get_speed(char_code: i8) -> f32 {
    match char_code {
        2 => 1.0,
        1 => 1.0,
        0 => 0.0,
        _ => 0.0,
    }
}

// This receives the user input and moves the player around
fn move_champ(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: QuerySet<(
        Query<&mut Transform, With<PlayerInfo>>,
        Query<(&mut Transform, &Camera)>,
    )>,
    mut player: ResMut<Player>,
) {
    for mut transform in query.q0_mut().iter_mut() {
        // Receiving the speed for this character
        let speed = get_speed(player.char_code);

        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += speed;
            player.y = transform.translation.y;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= speed;
            player.y = transform.translation.y;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= speed;
            player.x = transform.translation.x;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += speed;
            player.x = transform.translation.x;
        }
    }

    // This func is going to focus the camera on the character
    for (mut transform, camera) in query.q1_mut().iter_mut() {
        if camera.name == Some(CAMERA_3D.to_string()) {
            transform.translation.x = player.x;
            transform.translation.y = player.y;
            *transform = transform.looking_at(Vec3::from([player.x, player.y, 0.0]), Vec3::Y);
        }
    }
}
