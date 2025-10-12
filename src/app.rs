// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use bevy::prelude::*;
use bevy::time::Fixed;
use crate::player::PlayerPlugin;
use crate::physics::PhysicsPlugin;
use crate::rope::RopePlugin;
use crate::player::config::{PLAYER_INITIAL_POSITION, PLAYER_INITIAL_VELOCITY, PlayerSpawnPoint, PlayerSpawnVelocity};
use crate::event::ForceEvent;

pub fn run() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })
        .add_plugins(DefaultPlugins)
        .add_event::<ForceEvent>()
        .add_plugins(PlayerPlugin)
        .add_plugins(RopePlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera::default(),
    ));
}
