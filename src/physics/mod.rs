// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;

mod gravity;
mod integrate;
mod net_force;

mod config;
mod components;

pub(in crate::physics) use self::components::*;
pub(in crate::physics) use self::config::*;

use self::gravity::gravity_system;
use self::integrate::integrate_momentum_system;
use self::integrate::integrate_velocity_system;
use self::integrate::boundary;
use self::net_force::clean_force_system;
use self::net_force::collect_force_events_system;
use self::net_force::integrate_force_system;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate, 
            (
                clean_force_system,
                gravity_system,
                clean_rope_force_system,
                integrate_force_system,
                integrate_momentum_system,
                integrate_velocity_system,
                boundary,
            ).chain()
        );
    }
}