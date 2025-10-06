use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
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

    let task_pool = IoTaskPool::get();
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
                        continue;
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
    if data.len() < 5 {
        println!("[UDP Server] Malformed input event {:?}", data);
        return;
    }

    let seq = u32::from_be_bytes(data[0..4].try_into().unwrap());
    let mask = data[4];

    client.input.up    = mask & (1 << 0) != 0;
    client.input.left  = mask & (1 << 1) != 0;
    client.input.down  = mask & (1 << 2) != 0;
    client.input.right = mask & (1 << 3) != 0;

    client.last_seen = Instant::now();

    println!(
        "[UDP Server] seq={} mask={:04b} -> {:?}",
        seq, mask, client.input
    );
}
