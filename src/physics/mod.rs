// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;

pub mod integrate;
pub mod control;

use self::control::player_movement_input_system;
use self::integrate::clean_force_system;
use self::integrate::integrate_force_system;
use self::integrate::integrate_velocity_system;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate, 
            (
                clean_force_system,
                player_movement_input_system,
                integrate_force_system,
                integrate_velocity_system
            ).chain()
        );
    }
}