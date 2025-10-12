// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player configuration>

use bevy::prelude::*;

// Player control parameters
pub const PLAYER_CONTROL_SPEED_LIMIT: f32 = 10.0 * SCALE; // in pixel/s
pub const PLAYER_MOVE_FORCE: f32 = 1500.0 * SCALE; // in Newton = kg*pixel/s^2
pub const PLAYER_JUMP_FORCE: f32 = 20000.0 * SCALE; // in Newton

// Player spawn parameters
pub const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 64.0);
pub const PLAYER_INITIAL_POSITION: Vec3 = Vec3::new(-500.0, -200.0, 0.0);
pub const PLAYER_INITIAL_VELOCITY: Vec2 = Vec2::new(0.0, 0.0);

pub const PLAYER_SPAWN_MASS: f32 = 120.0; // in kg

#[derive(Resource, Clone, Copy)]
pub struct PlayerSpawnPoint {
    pub position: Vec3,
}

#[derive(Resource, Clone, Copy)]
pub struct PlayerSpawnVelocity {
    pub velocity: Vec2,
}