# Barrels

This is the Barrels client written in Rust using the Bevy framework

This client implements a UDP client to comunicate with another clients via the `BarrelsServer`

I start the UDP client when the game is initialized
and i use it to constantly send my player data and to receiv the other players data

```rust
let socket = net::UdpSocket::bind("127.0.0.1:988").expect("");
socket.set_nonblocking(true).expect("Non Blocking failed");
let server = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 9999);
```

with the socket initialized and the server we can now send packets

```rust
let my_player = Player{};
socket.send_to(my_player.as_bytes(), server);
```

and receiv packets

```rust
let mut buf = [0; 1024];

// handle the error with a match
socket.recv_from(&mut buf);
```

With this functionalities we can work on our game as much as we want!
