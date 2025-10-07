// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use bevy::prelude::*;
use bevy::time::Fixed;
use crate::player::PlayerPlugin;
use crate::physics::PhysicsPlugin;
use crate::config::*;

use crate::map::{MapPlugin, SCREEN};
use crate::util::{DevModePlugin};
use crate::game_ui::{UIPlugin};

use crate::physics::rope_force::{apply_rope_geometry, init_ropes, RopeGeometry, rope_tension_system, rope_force_to_system, compute_rope_geometry};
use crate::player::player_plugin::spawn_player;
// <- compute_rope_geometry 删除了

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
            translation: Vec3::new(
                 SCREEN.0 / 2.0,
                 SCREEN.1 / 2.0,
                 0.0,
             ),
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

pub fn run() {
    let mut app = App::new();
    #[cfg(debug_assertions)]
    app.add_plugins(DevModePlugin);

    app
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })

        .add_systems(Startup, init_player_camera)

        .add_plugins(MapPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UIPlugin)

        .add_systems(Update, update_camera)
        .insert_resource(RopeGeometry::default())

        // .add_systems(Startup, init_ropes)
        .add_systems(Startup, init_ropes.after(spawn_player))
        .add_systems(Update, rope_tension_system)
        .add_systems(Update, rope_force_to_system)
        .add_systems(Update, compute_rope_geometry)

        .add_systems(Update, apply_rope_geometry)
        
        .run();
}
