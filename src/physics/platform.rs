// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Platform Moving Logic>
use bevy::prelude::*;

use super::config::{PLATFORM_RESTITUTION, PLATFORM_FRICTION};
use super::physics_core::component::RigidBody;
use crate::event::Platform2PhysicsAttach;

pub(super) fn platform_insert_physics(
    mut commands: Commands,
    mut events: EventReader<Platform2PhysicsAttach>,
) {
    let platform_restitution = PLATFORM_RESTITUTION;
    let platform_friction = PLATFORM_FRICTION;
    for event in events.read() {
        commands.entity(event.entity).insert(
            RigidBody {
                inv_mass: event.inv_mass,
                restitution: platform_restitution,
                friction: platform_friction,
            },
        );
    }
}