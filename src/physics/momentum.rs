// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Net force module>
use bevy::prelude::*;
use super::component::{Momentum, NetForce};
use crate::event::{ImpulseEvent};

pub(super) fn integrate_force_system(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Momentum, &NetForce)>,
) {
    let delta_seconds = time.delta_secs();
    for (mut momentum, net_force) in query.iter_mut() {
        momentum.0 += net_force.0 * delta_seconds;
    }
}

pub(super) fn collect_impulse_event_system(
    mut events: EventReader<ImpulseEvent>,
    mut query: Query<&mut Momentum>,
) {
    for event in events.read() {
        if let Ok(mut momentum) = query.get_mut(event.target) {
            momentum.0 += event.impulse;
        } 
    }
}