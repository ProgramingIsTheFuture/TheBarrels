use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    render::{camera::Camera, render_graph::base::camera::CAMERA_3D},
};

use crate::player::types::Player;

const SPRITE_FRONT: i8 = 3; // 0 - 3
const SPRITE_BACK: i8 = 7; // 4 - 7
const SPRITE_RIGHT: i8 = 11; // 8 - 11
const SPRITE_LEFT: i8 = 15; // 12 - 15

pub struct PlayerInfo;

pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("Spawn player", SystemStage::single(setup_player.system()))
            .add_system(move_champ.system())
            .add_system(animate_sprite_system.system());
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(Player::default());

    let h = TextureAtlas::from_grid(
        asset_server.load("sprites/pirate_1.png"),
        Vec2::new(16.0, 16.0),
        4,
        4,
    );

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(h),
            transform: Transform {
                translation: Vec3::new(0., 0., 3.),
                scale: Vec3::new(0.2, 0.2, 0.2),
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
    if !player.is_changed() {
        player.moving = false;
    }

    for mut transform in query.q0_mut().iter_mut() {
        // Receiving the speed for this character
        let speed = get_speed(player.char_code);

        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += speed;
            player.y = transform.translation.y;
            player.direction = SPRITE_BACK;
            player.moving = true;
            continue;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= speed;
            player.y = transform.translation.y;
            player.direction = SPRITE_FRONT;
            player.moving = true;
            continue;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= speed;
            player.x = transform.translation.x;
            player.direction = SPRITE_LEFT;
            player.moving = true;
            continue;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += speed;
            player.x = transform.translation.x;
            player.direction = SPRITE_RIGHT;
            player.moving = true;
            continue;
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

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
    player: Res<Player>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let move_dir = player.direction;

            if !player.moving {
                sprite.index = (move_dir - 3) as u32;
                continue;
            }

            if (move_dir - 3) > (sprite.index as i8) || (sprite.index as i8) > move_dir {
                sprite.index = (move_dir - 3) as u32;
                continue;
            }
            if move_dir == (sprite.index as i8) {
                sprite.index = (move_dir - 3) as u32;
                continue;
            }
            sprite.index += 1;
            continue;
        }
    }
}
