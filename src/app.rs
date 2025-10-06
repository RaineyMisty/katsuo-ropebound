// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use crate::config::*;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::sprite::SpritePlugin;
use bevy::time::Fixed;
use std::env;

use crate::map::{MapPlugin, SCREEN};
use crate::util::DevModePlugin;
use crate::multiplayer::UdpServerPlugin;
use crate::multiplayer::UdpClientPlugin;

// Example query for getting the platform colliders which are visible on screen
// fn log_offscreen_entities(
//     q: Query<(Entity, &ViewVisibility), (With<Collider>, With<Transform>)>,
// ) {
//     for (e, view) in &q {
//         if !view.get() {
//             info!("🛰 Entity {:?} with Collider is off-screen", e);
//         }
//     }
// }

// move a half screen right and a half screen up.
// so that the origin is in the positive coordinate system
fn init_player_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 0, // draw first (background/world)
            ..default()
        },
        Transform {
            translation: Vec3::new(SCREEN.0 / 2.0, SCREEN.1 / 2.0, 0.0),
            ..Default::default()
        },
        MainCamera,
    ));
}

// Camera Components
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FollowedPlayer;

const CAMERA_DECAY_RATE: f32 = 3.;

// System for the camera movement
fn update_camera(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<FollowedPlayer>)>,
    player: Single<&Transform, (With<FollowedPlayer>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { y, .. } = player.translation;

    let min_y = SCREEN.1 / 2.0;
    let clamped_y = y.max(min_y);
    let target = Vec3::new(camera.translation.x, clamped_y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());
}

#[derive(Resource)]
struct SimulateKeyTimer(Timer);

fn simulate_keys(
    time: Res<Time>,
    mut timer: ResMut<SimulateKeyTimer>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
) {
    // Tick the timer each frame
    timer.0.tick(time.delta());

    if !timer.0.finished() {
        // While timer is still running, press the keys
        keys.press(KeyCode::ArrowRight);
        keys.press(KeyCode::KeyD);
    } else {
        // Once the timer is done, release the keys once
        keys.release(KeyCode::ArrowRight);
        keys.release(KeyCode::KeyD);
    }
}

pub fn setup_timer(mut commands: Commands) {
    commands.insert_resource(SimulateKeyTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

pub fn run() {
    let mut app = App::new();
    #[cfg(debug_assertions)] // not added in release mode.
    app.add_plugins(DevModePlugin);

    #[cfg(feature = "client")]
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(UdpClientPlugin {
            server_addr: "127.0.0.1:5000".to_string(),
        });
    #[cfg(feature = "server")]
    app.add_plugins(MinimalPlugins)
       .add_plugins(UdpServerPlugin)
       .add_plugins(bevy::input::InputPlugin)
       .add_systems(Startup, setup_timer)
       .add_systems(Update, simulate_keys);

    app.insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint {
            position: PLAYER_INITIAL_POSITION,
        })
        .insert_resource(PlayerSpawnVelocity {
            velocity: PLAYER_INITIAL_VELOCITY,
        })
        .add_systems(Startup, init_player_camera)
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Update, update_camera)
        .run();
}
