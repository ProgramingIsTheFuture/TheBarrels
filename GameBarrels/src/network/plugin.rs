use bevy::prelude::*;
use std::io;
use std::net;

use crate::player::plugin::PlayerInfo;
use crate::player::types::{Player, PlayerType};

pub struct Network {
    pub socket: net::UdpSocket,
    pub server: net::SocketAddr,
}

impl Default for Network {
    fn default() -> Self {
        let socket = net::UdpSocket::bind("127.0.0.1:8888").expect("");
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
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let p: Player = Player {
                username: player.username.to_string(),
                id: player.id.to_string(),
                x: player.x,
                y: player.y,
                char_code: player.char_code,
            };
            let j = serde_json::to_string(&p).unwrap();

            println!("Sending my data");

            match network.socket.send_to(j.as_bytes(), network.server) {
                Ok(_) => return,
                Err(_) => return,
            };
        }
    }
}

fn recv_data(
    mut commands: Commands,
    player_type: Res<PlayerType>,
    mut query: Query<(&mut Player, &mut Transform)>,
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
    let player: Player = match serde_json::from_str(receiv) {
        Ok(v) => v,
        Err(_) => return,
    };

    // This check if the user exists and update it
    for (mut p, mut transform) in query.iter_mut() {
        if p.id == player.id {
            println!(
                "Updating - {} - {} - {}",
                player.x, player.y, player.username
            );
            transform.translation.x = player.x;
            transform.translation.y = player.y;
            p.x = player.x;
            p.y = player.y;

            return;
        }
        continue;
    }

    // Spwan a new user
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
        .insert(player);
}
