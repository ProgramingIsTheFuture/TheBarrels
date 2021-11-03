use bevy::prelude::*;
use std::io;
use std::net;

use crate::player::plugin::PlayerInfo;
use crate::player::types::{Action, Player};

pub struct Network {
    pub socket: net::UdpSocket,
    pub server: net::SocketAddr,
}

impl Default for Network {
    fn default() -> Self {
        let socket = net::UdpSocket::bind("127.0.0.1:7777").expect("");
        socket.set_nonblocking(true).expect("Non Blocking failed");
        let server = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 9999);
        Self { socket, server }
    }
}

pub struct NetworkPlugin {}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(start_network.system())
            .add_system(send_data.system())
            .add_system(recv_data.system());
    }
}

// insert the network resource
fn start_network(mut commands: Commands) {
    commands.insert_resource(Network::default());
}

fn send_data(
    network: ResMut<Network>,
    player: Res<Player>,
    time: Res<Time>,
    mut query: Query<&mut Timer, With<PlayerInfo>>,
) {
    if !player.is_changed() {
        return;
    }
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let p: Player = Player {
                username: player.username.to_string(),
                id: player.id.to_string(),
                x: player.x,
                y: player.y,
                direction: player.direction,
                moving: player.moving,
                char_code: player.char_code,
            };
            let data: Action<Player> = Action {
                action: "move".to_string(),
                data: p,
            };
            let j = serde_json::to_string(&data).unwrap();

            match network.socket.send_to(j.as_bytes(), network.server) {
                Ok(_) => return,
                Err(_) => return,
            };
        }
    }
}

fn recv_data(
    mut commands: Commands,
    mut query: QuerySet<(Query<(&mut Player, &mut Transform)>, Query<&Player>)>,
    // query_remove: Query<(Entity), With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    network: ResMut<Network>,
) {
    let mut buf = [0; 1024];

    // This will try to receiv data
    // if the data is not ready, this wont block
    // if happends an error, it will just ignore it
    let (size, _) = loop {
        match network.socket.recv_from(&mut buf) {
            Ok(n) => break n,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return,
            Err(_e) => return,
        }
    };

    // Convert bytes to string
    let receiv = std::str::from_utf8(&buf[..size]).unwrap();

    // Deserialize the data as string
    let action: Action<Player> = match serde_json::from_str(receiv) {
        Ok(v) => v,
        Err(_) => return,
    };

    let player: Player = action.data;

    let mut spawn: bool = true;
    for p in query.q1().iter() {
        if p.id == player.id {
            spawn = false;
            break;
        }
        spawn = true;
        continue;
    }

    if spawn == true {
        let h = TextureAtlas::from_grid(
            asset_server.load("sprites/pirate_1.png"),
            Vec2::new(16.0, 16.0),
            4,
            4,
        );

        // Spwan a new user
        let texture = texture_atlases.add(h);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture,
                transform: Transform {
                    translation: Vec3::new(0., 0., 3.),
                    scale: Vec3::new(0.2, 0.2, 0.2),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Timer::from_seconds(0.05, true))
            .insert(player);
        return;
    }

    if action.action == "move" {
        // This check if the user exists and update it
        for (mut p, mut transform) in query.q0_mut().iter_mut() {
            if p.id == player.id {
                transform.translation.x = player.x;
                transform.translation.y = player.y;
                p.x = player.x;
                p.y = player.y;
                p.direction = player.direction;
                p.moving = player.moving;

                return;
            }
            continue;
        }
    }

    /*
    if action.action == "disconnected" {
        for ent in query_remove.iter() {}
    }
    */
}
