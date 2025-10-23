// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;

mod gravity;
mod integrate;
mod momentum;
mod net_force;

pub(super) mod bundle;
pub(super) mod config;
pub(super) mod component;

use self::gravity::gravity_system;
use self::integrate::integrate_momentum_system;
use self::integrate::integrate_velocity_system;
use self::integrate::boundary;
use self::momentum::integrate_force_system;
use self::momentum::collect_impulse_event_system;
use self::net_force::clean_force_system;
use self::net_force::collect_force_events_system;

pub(super) struct PhysicsCorePlugin;
impl Plugin for PhysicsCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate, 
            (
                clean_force_system,
                gravity_system,
                collect_force_events_system,
                integrate_force_system,
                collect_impulse_event_system,
                integrate_momentum_system,
                integrate_velocity_system,
                boundary,
            ).chain()
        );
    }
}