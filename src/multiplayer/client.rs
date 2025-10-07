use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use std::net::UdpSocket;
use std::time::Duration;

/// Resource to hold the client socket after handshake
#[derive(Resource)]
pub struct UdpClientSocket {
    pub socket: UdpSocket,
    pub server_addr: std::net::SocketAddr,
}


#[derive(Debug)]
pub enum InputAction {
    Pressed,
    Released,
}

#[derive(Event, Debug)]
pub struct ClientInputEvent {
    pub key_id: u8,           // 0=W, 1=A, 2=S, 3=D
    pub action: InputAction,
    pub sequence: u32,
}

pub fn keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input_events: EventWriter<ClientInputEvent>,
    mut seq_counter: Local<u32>,
) {
    let keys = [
        (KeyCode::KeyW, 0),
        (KeyCode::KeyA, 1),
        (KeyCode::KeyS, 2),
        (KeyCode::KeyD, 3),
    ];

    for (code, id) in keys {
        if keyboard.just_pressed(code) {
            *seq_counter += 1;
            input_events.write(ClientInputEvent {
                key_id: id,
                action: InputAction::Pressed,
                sequence: *seq_counter,
            });
        }

        if keyboard.just_released(code) {
            *seq_counter += 1;
            input_events.write(ClientInputEvent {
                key_id: id,
                action: InputAction::Released,
                sequence: *seq_counter,
            });
        }
    }
}

pub fn send_input_state_system(
    mut seq: Local<u32>,
    keyboard: Res<ButtonInput<KeyCode>>,
    client: Option<Res<UdpClientSocket>>,
) {
    if client.is_none() { return; }
    let client = client.unwrap();

    let mut mask = 0u8;
    if keyboard.pressed(KeyCode::KeyW) { mask |= 1 << 0; }
    if keyboard.pressed(KeyCode::KeyA) { mask |= 1 << 1; }
    if keyboard.pressed(KeyCode::KeyS) { mask |= 1 << 2; }
    if keyboard.pressed(KeyCode::KeyD) { mask |= 1 << 3; }

    *seq += 1;
    let mut buf = Vec::with_capacity(5);
    buf.extend_from_slice(&seq.to_be_bytes());
    buf.push(mask);

    if let Err(e) = client.socket.send_to(&buf, client.server_addr) {
        eprintln!("[Client] Failed to send input state: {}", e);
    }
}

/// Resource to temporarily store the server address before handshake
#[derive(Resource)]
pub struct ServerAddress(pub String);

pub fn client_handshake(mut commands: Commands, server_addr: Res<ServerAddress>) {
    let server_addr: std::net::SocketAddr = server_addr
        .0
        .parse()
        .expect("Failed to parse server address");

    // Create UDP socket and bind to a random available port on localhost
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP client socket");
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("Failed to set read timeout");

    println!("[Client] Sending HELLO to {}", server_addr);
    socket
        .send_to(b"HELLO", server_addr)
        .expect("Failed to send HELLO");

    let mut buf = [0u8; 1024];
    match socket.recv_from(&mut buf) {
        Ok((len, addr)) => {
            let msg = &buf[..len];
            if msg == b"ACK" {
                println!("[Client] Handshake successful with server {}", addr);
                commands.insert_resource(UdpClientSocket {
                    socket,
                    server_addr,
                });
            } else {
                println!("[Client] Unexpected handshake response: {:?}", msg);
            }
        }
        Err(e) => {
            eprintln!("[Client] Handshake failed: {}", e);
        }
    }
}
