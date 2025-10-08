use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use std::net::UdpSocket;
use std::time::Duration;
use async_channel::{Sender, Receiver};

use crate::{app::{IsMainPlayer, MainPlayer}, player::Player};

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

#[derive(Debug)]
pub struct SnapshotUpdate {
    pub tick: u32,
    pub positions: Vec<(f32, f32)>,
}

#[derive(Resource)]
pub struct ClientNetChannels {
    pub rx_snapshots: Receiver<SnapshotUpdate>,
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

pub fn client_handshake(mut commands: Commands, server_addr: Res<ServerAddress>, is_main_player: Res<IsMainPlayer>) {
    let server_addr: std::net::SocketAddr = server_addr
        .0
        .parse()
        .expect("Failed to parse server address");

    // Create UDP socket and bind to a random available port on localhost
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP client socket");
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("Failed to set read timeout");

    let (tx_snapshots, rx_snapshots) = async_channel::unbounded::<SnapshotUpdate>();
    let tx_snapshots_clone = tx_snapshots.clone();

    println!("[Client] Sending HELLO to {}", server_addr);

    let msg = if is_main_player.as_ref().0 {
        b"MAIN"
    } else {
        b"PLAY"
    };

    socket
        .send_to(msg, server_addr)
        .expect("Failed to send handshake message");

    let mut buf = [0u8; 1024];
    match socket.recv_from(&mut buf) {
        Ok((len, addr)) => {
            let msg = &buf[..len];
            if msg == b"ACK" {
                println!("[Client] Handshake successful with server {}", addr);

                // Clone socket so it can live in both sending and receiving tasks
                let socket_clone = socket.try_clone().expect("Failed to clone client socket");

                // Spawn async task to receive snapshots
                let task_pool = IoTaskPool::get();
                task_pool.spawn(async move {
                    let mut buf = [0u8; 1500];
                    loop {
                        match socket_clone.recv_from(&mut buf) {
                            Ok((len, from)) => {
                                let snapshot = &buf[..len];
                                if snapshot.len() < 6 {
                                    eprintln!("[Client] Invalid snapshot length {}", snapshot.len());
                                    continue;
                                }

                                let tick = u32::from_be_bytes(snapshot[0..4].try_into().unwrap());
                                let player_count = u16::from_be_bytes(snapshot[4..6].try_into().unwrap()) as usize;

                                let mut offset = 6;
                                println!("Tick {} with {} players", tick, player_count);

                                let mut positions = Vec::with_capacity(player_count);

                                for i in 0..player_count {
                                    if offset + 8 > snapshot.len() {
                                        eprintln!("[Client] Truncated snapshot for player {}", i);
                                        break;
                                    }

                                    let x = f32::from_be_bytes(snapshot[offset..offset+4].try_into().unwrap());
                                    let y = f32::from_be_bytes(snapshot[offset+4..offset+8].try_into().unwrap());
                                    offset += 8;

                                    println!("  Player {}: x={:.1}, y={:.1}", i, x, y);

                                    positions.push((x, y));
                                }
                                if let Err(e) = tx_snapshots_clone.try_send(SnapshotUpdate { tick, positions }) {
                                    eprintln!("[Client] Failed to enqueue snapshot: {}", e);
                                }
                            }

                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                std::thread::yield_now();
                                continue;
                            }
                            Err(e) => {
                                eprintln!("[Client] Snapshot recv error: {}", e);
                                break;
                            }
                        }
                    }
                }).detach();

                commands.insert_resource(UdpClientSocket {
                    socket,
                    server_addr,
                });
                commands.insert_resource(ClientNetChannels { rx_snapshots });
            }
        }
        Err(e) => {
            eprintln!("[Client] Handshake failed: {}", e);
        }
    }
}

pub fn apply_snapshot_system(
    channels: Res<ClientNetChannels>,
    mut main_query: Query<&mut Transform, (With<Player>, With<MainPlayer>)>,
    mut other_query: Query<&mut Transform, (With<Player>, Without<MainPlayer>)>,
) {
    while let Ok(snapshot) = channels.rx_snapshots.try_recv() {
        if snapshot.positions.is_empty() {
            continue;
        }

        if let Ok(mut main_transform) = main_query.single_mut() {
            if let Some((x, y)) = snapshot.positions.get(0) {
                main_transform.translation.x = *x;
                main_transform.translation.y = *y;
            }
        }

        if let Ok(mut other_transform) = other_query.single_mut() {
            if let Some((x, y)) = snapshot.positions.get(1) {
                other_transform.translation.x = *x;
                other_transform.translation.y = *y;
            }
        }
    }
}
