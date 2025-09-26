// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics configuration>

use bevy::prelude::*;

// Gravity constant
pub const GRAVITY: Vec2 = Vec2::new(0.0, -98.0);

// Player control parameters
pub const PLAYER_CONTROL_SPEED_LIMIT: f32 = 120.0;
pub const PLAYER_MOVE_FORCE: f32 = 3000.0;
pub const PLAYER_JUMP_FORCE: f32 = 6000.0;

// Rope parameters
pub const ROPE_REST_LENGTH: f32 = 300.0;
pub const ROPE_MAX_EXTENSION: f32 = 50.0;  // Maximum extension beyond rest length
pub const SPRING_CONSTANT: f32 = 50.0;