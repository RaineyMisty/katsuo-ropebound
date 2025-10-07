use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct ClientSession {
    pub last_seen: Instant,
    // You can add player_id, entity, etc. later if needed
}

#[derive(Resource, Default, Clone)]
pub struct ClientRegistry {
    pub clients: Arc<RwLock<HashMap<SocketAddr, ClientSession>>>,
}

#[derive(Resource)]
pub struct UdpServerSocket {
    pub socket: UdpSocket,
}

pub fn setup_udp_server(mut commands: Commands) {
    let socket = UdpSocket::bind("0.0.0.0:5000").expect("Failed to bind UDP socket");
    socket.set_nonblocking(true).unwrap();
    println!("[UDP Server] Listening on 0.0.0.0:5000");

    let registry = ClientRegistry::default();
    let socket_clone = socket.try_clone().unwrap();

    commands.insert_resource(UdpServerSocket { socket });
    commands.insert_resource(registry.clone());

    let task_pool = IoTaskPool::get();

    // Recieve from client
    {
        let recv_socket = socket_clone.try_clone().unwrap();
        let recv_clients = registry.clone();
        task_pool.spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                match recv_socket.recv_from(&mut buf) {
                    Ok((len, addr)) => {
                        let data = &buf[..len];

                        if data == b"HELLO" {
                            handle_handshake(&recv_socket, &recv_clients, addr);
                        } else {
                            handle_client_input(&recv_clients, addr, data);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::yield_now();
                        continue;
                    }
                    Err(e) => {
                        eprintln!("[UDP Server] Recv error: {:?}", e);
                        break;
                    }
                }
            }
        }).detach();
    }

    // Broadcast to all clients task (20 Hz)
    {
        let broadcast_socket = socket_clone;
        let broadcast_clients = registry.clone();
        task_pool.spawn(async move {
            let tick_rate = Duration::from_millis(50);
            let mut tick: u32 = 0;
            loop {
                std::thread::sleep(tick_rate);
                tick += 1;

                let snapshot_data = build_snapshot_packet(tick);

                let clients_guard = broadcast_clients.clients.read().unwrap();
                for addr in clients_guard.keys() {
                    if let Err(e) = broadcast_socket.send_to(&snapshot_data, addr) {
                        eprintln!("[UDP Server] Failed to send snapshot to {}: {}", addr, e);
                    }
                }
            }
        }).detach();
    }
}

/// Example snapshot payload
fn build_snapshot_packet(tick: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&tick.to_be_bytes());
    buf.extend_from_slice(b"SNAPSHOT");
    buf
}

fn handle_handshake(socket: &UdpSocket, registry: &ClientRegistry, addr: SocketAddr) {
    println!("[UDP Server] Handshake from {}", addr);
    let mut map = registry.clients.write().unwrap();

    map.insert(
        addr,
        ClientSession {
            last_seen: Instant::now(),
        },
    );

    let _ = socket.send_to(b"ACK", addr);
}

fn handle_client_input(registry: &ClientRegistry, addr: SocketAddr, data: &[u8]) {
    let clients_read = registry.clients.read().unwrap();
    if clients_read.contains_key(&addr) {
        // Here you'd parse and immediately apply input to the ECS world,
        // instead of storing it on the session.
        handle_input_event(data);
    } else {
        println!("[UDP Server] Unknown client {} sent {:?}", addr, data);
    }
}

/// Immediate input processing
fn handle_input_event(data: &[u8]) {
    if data.len() < 5 {
        println!("[UDP Server] Malformed input event {:?}", data);
        return;
    }

    let seq = u32::from_be_bytes(data[0..4].try_into().unwrap());
    let mask = data[4];

    let up    = mask & (1 << 0) != 0;
    let left  = mask & (1 << 1) != 0;
    let down  = mask & (1 << 2) != 0;
    let right = mask & (1 << 3) != 0;

    println!(
        "[UDP Server] seq={} mask={:04b} (up={} left={} down={} right={})",
        seq, mask, up, left, down, right
    );

    // Apply to authoritative ECS here if needed.
}
