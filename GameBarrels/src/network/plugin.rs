use bevy::prelude::*;
use std::io;
use std::net;

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

fn start_network(mut commands: Commands) {
    commands.insert_resource(Network::default());
}

fn send_data(network: ResMut<Network>) {
    //let buf = [0; 10];

    let ab = "Hello world".as_bytes();
    network.socket.send_to(ab, network.server);
}

fn recv_data(network: ResMut<Network>) {
    let mut buf = [0; 10];
    // let size = network.socket.recv(&mut buf).expect("");

    let (size, _) = loop {
        match network.socket.recv_from(&mut buf) {
            Ok(n) => break n,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return,
            Err(e) => panic!("encountered IO error: {}", e),
        }
    };
}
