// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for physics integration>
use bevy::prelude::*;
use crate::components::motion::{Velocity, NetForce};

pub fn integrate_force_system(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Velocity, &NetForce)>,
) {
    let delta_seconds = time.delta_secs();
    for (mut velocity, net_force) in query.iter_mut() {
        velocity.0.x += net_force.0.x * delta_seconds;
        velocity.0.y += net_force.0.y * delta_seconds;
    }
}

pub fn integrate_velocity_system(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    let delta_seconds = time.delta_secs();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * delta_seconds;
        transform.translation.y += velocity.0.y * delta_seconds;
    }
}