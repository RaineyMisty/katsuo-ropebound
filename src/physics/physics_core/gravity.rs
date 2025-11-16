// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Gravity system>
use bevy::prelude::*;
use super::config::GRAVITY;
use super::component::{NetForce, Gravity, Mass};
use crate::collision::component::OnGround;

pub(super) fn on_ground(
    mut query: Query<(&OnGround, &mut Gravity)>,
) {
    for (on_ground, mut gravity) in query.iter_mut() {
        gravity.0 = if on_ground.0 == true {
            false
        } else {
            true
        }
    }
}

pub(super) fn gravity_system(
    mut query: Query<(Entity, &mut NetForce, &Gravity, &Mass)>,
) {
    for (e, mut net_force, gravity, mass) in &mut query {
        // info!("E {} is no gravity? {}", e, gravity.0);
        if gravity.0 {
            // F = m * g
            net_force.0 += GRAVITY * mass.0;
        }
    }
}