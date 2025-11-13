// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Collision physics>
use bevy::prelude::*;

use crate::event::Collision2PhysicsInfo;

pub(super) fn collision_info_to_impulse (
    mut events: EventReader<Collision2PhysicsInfo>,
) {
    for event in events.read() {
        info!("Entity {} and {} collide, on {} and {}.", event.entity_a, event.entity_b, event.normal, event.penetration);
    }
}