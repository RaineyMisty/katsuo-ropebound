// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player physics>
use bevy::prelude::*;

use super::config::{PLAYER_RESTITUTION, PLAYER_FRICTION};
use super::physics_core::bundle::PhysicsBundle;
use super::physics_core::component::RigidBody;
use crate::event::Player2PhysicsAttach;

pub(super) fn player_insert_physics(
    mut commands: Commands,
    mut events: EventReader<Player2PhysicsAttach>,
) {
    let player_restitution = PLAYER_RESTITUTION;
    let player_friction = PLAYER_FRICTION;
    for event in events.read() {
        commands.entity(event.entity).insert(
            PhysicsBundle::new(event.mass, true)
        );
        if event.mass != 0.0 {
            commands.entity(event.entity).insert(
                RigidBody {
                    inv_mass: 1.0 / event.mass,
                    restitution: player_restitution,
                    friction: player_friction,
                },
            );
        }
    }
}