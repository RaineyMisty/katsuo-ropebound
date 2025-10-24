// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player constraint in physics>
use bevy::prelude::*;

use super::physics_core::component::{Velocity, NetForce, Impulse};
use super::config::{PLAYER_CONTROL_SPEED_LIMIT, PLAYER_MOVE_FORCE, PLAYER_JUMP_IMPULSE};

use crate::event::{PlayerIntentEvent, PlayerIntentKind};

pub(super) fn player_intent_to_force(
    mut intent_events: EventReader<PlayerIntentEvent>,
    mut query: Query<(&Velocity, &mut NetForce, &mut Impulse)>,
) {
    let speed_limit = PLAYER_CONTROL_SPEED_LIMIT;
    let move_force = PLAYER_MOVE_FORCE;
    let jump_impulse = PLAYER_JUMP_IMPULSE;

    for event in intent_events.read() {
        let (velocity, mut net_force, mut impulse) = match query.get_mut(event.player) {
            Ok(data) => data,
            Err(_) => continue,
        };

        let mut force_limit = Vec2::ZERO;
        let resistance_constant = move_force / speed_limit;
        let mut jump_impulse = Vec2::ZERO;
        match event.intent {
            PlayerIntentKind::Move { axis_x } => {
                let speed = velocity.0.x * axis_x.signum();
                if speed <= 0.0 {
                    force_limit.x = move_force * axis_x.signum();
                } else if speed < speed_limit {
                    let resistance = resistance_constant * speed;
                    force_limit.x = (move_force - resistance) * axis_x.signum();
                } else {
                    force_limit.x = 0.0;
                }
            },
            PlayerIntentKind::JumpStart => {
                jump_impulse.y = PLAYER_JUMP_IMPULSE;
            },
            PlayerIntentKind::JumpHold { dt: _ } => {
                // no continuous force for now
            },
            PlayerIntentKind::JumpEnd => {
                // no end force for now
            },
        }
        net_force.0 += force_limit;
        impulse.0 += jump_impulse;
    }
}