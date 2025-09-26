// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Gravity system>
use bevy::prelude::*;
use crate::config::physics::GRAVITY;
use crate::components::motion::{NetForce, Gravity};

pub fn gravity_system(
    mut query: Query<(&mut NetForce, &Gravity)>,
) {
    for (mut net_force, gravity) in &mut query {
        if gravity.0 {
            net_force.0 += GRAVITY;
        }
    }
}