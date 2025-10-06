use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::{Duration, Instant},
};

#[derive(Debug, Default, Clone)]
pub struct InputState {
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
}

#[derive(Debug)]
pub struct ClientSession {
    pub last_seen: Instant,
    pub input: InputState,
}

#[derive(Resource, Default)]
pub struct ClientRegistry {
    pub clients: HashMap<SocketAddr, ClientSession>,
}

pub struct UdpServerPlugin;

impl Plugin for UdpServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClientRegistry::default())
            .add_systems(Startup, setup_udp_server);
    }
}

fn setup_udp_server(mut commands: Commands) {
    let socket = UdpSocket::bind("0.0.0.0:5000").expect("Failed to bind UDP socket");
    socket.set_nonblocking(true).unwrap();
    println!("[UDP Server] Listening on 0.0.0.0:5000");

    let socket_clone = socket.try_clone().unwrap();
    commands.insert_resource(UdpServerSocket { socket });

    let task_pool = AsyncComputeTaskPool::get();
    task_pool
        .spawn(async move {
            let mut buf = [0u8; 1024];
            let mut clients: HashMap<SocketAddr, ClientSession> = HashMap::new();

            loop {
                match socket_clone.recv_from(&mut buf) {
                    Ok((len, addr)) => {
                        let data = &buf[..len];

                        if data == b"HELLO" {
                            handle_handshake(&socket_clone, &mut clients, addr);
                        } else if let Some(client) = clients.get_mut(&addr) {
                            handle_input_event(data, client);
                        } else {
                            println!("[UDP Server] Unknown client {} sent {:?}", addr, data);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(2));
                    }
                    Err(e) => {
                        eprintln!("[UDP Server] Error: {:?}", e);
                        break;
                    }
                }
            }
        })
        .detach();
}

#[derive(Resource)]
pub struct UdpServerSocket {
    pub socket: UdpSocket,
}

fn handle_handshake(socket: &UdpSocket, clients: &mut HashMap<SocketAddr, ClientSession>, addr: SocketAddr) {
    println!("[UDP Server] Handshake from {}", addr);
    clients.insert(addr, ClientSession { last_seen: Instant::now(), input: InputState::default() });
    let _ = socket.send_to(b"ACK", addr);
}

fn handle_input_event(data: &[u8], client: &mut ClientSession) {
    if data.len() < 6 {
        println!("[UDP Server] Malformed input event {:?}", data);
        return;
    }

    let seq = u32::from_be_bytes(data[0..4].try_into().unwrap());
    let key_id = data[4];
    let action = data[5]; // 1 = press, 0 = release

    // Update input state
    match key_id {
        0 => client.input.up = action == 1,
        1 => client.input.left = action == 1,
        2 => client.input.down = action == 1,
        3 => client.input.right = action == 1,
        _ => println!("[UDP Server] Unknown key id {}", key_id),
    }

    client.last_seen = Instant::now();
    println!(
        "[UDP Server] seq={} key={} action={} -> {:?}",
        seq, key_id, action, client.input
    );
}
