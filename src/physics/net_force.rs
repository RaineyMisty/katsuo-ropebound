// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Net force module>
use bevy::prelude::*;
use super::component::NetForce;
use crate::event::{ForceEvent};


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
    for event in events.read() {
        if let Ok(mut net_force) = query.get_mut(event.target) {
            
        }
    }
}
