// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use bevy::prelude::*;
use bevy::time::Fixed;
use crate::player::PlayerPlugin;
use crate::physics::PhysicsPlugin;
use crate::config::*;

use crate::util::map::loader::{load_map_resouces, load_map};
use crate::util::map::MapFile;
// use crate::util::dev_mode::move_camera_with_arrows;

use crate::util::map::Collider;
use bevy::color::Color;


pub fn draw_colliders(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Collider)>,
) {
    for (transform, collider) in &query {
        // Center of the rectangle in 2D
        let position_2d = transform.translation.truncate() + collider.offset;

        // Draw a rectangle centered on the entity's position
        gizmos.rect_2d(
            position_2d,
            collider.size,
            Color::srgba(1.0, 1.0, 1.0, 0.8),
        );
    }
}

// Component used for controlling camera in dev_mode
#[derive(Component)]
pub struct CameraController;

fn camera_start(mut commands: Commands, map: Res<MapFile>) {
    let screen = &map.metadata.screen_size;
    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(
                 screen.w as f32 / 2.0,
                 screen.h as f32 / 2.0,
                 0.0,
             ),
             scale: Vec3::splat(1.0),
             ..Default::default() 
        },
        CameraController,
    ));
}

pub fn run() {
    App::new()
        .add_systems(Startup, (load_map_resouces, camera_start, load_map).chain())
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .run();
}

