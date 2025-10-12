// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Net force module>
use bevy::prelude::*;
use self::components::NetForce;

use crate::event::{ForceEvent, ForceKind};


pub(super) fn clean_force_system(
    mut query: Query<&mut NetForce>,
) {
    for mut net_force in query.iter_mut() {
        net_force.0 = Vec2::ZERO;
    }
}

pub(super) fn collect_force_events_system(
    mut events: EventReader<ForceEvent>,
    mut query: Query<&mut NetForce>,
) {
    for event in events.iter() {
        if let Ok(mut net_force) = query.get_mut(event.target) {
            net_force.0 += event.force;
        }
    }
}

pub(super) fn integrate_force_system(
    time: Res<Time>,
    mut query: Query<(&mut Momentum, &NetForce)>,
) {
    let delta_seconds = time.delta_secs();
    for (mut momentum, net_force) in query.iter_mut() {
        momentum.0 += net_force.0 * delta_seconds;
    }
}