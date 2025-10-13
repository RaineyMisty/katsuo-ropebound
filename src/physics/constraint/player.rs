// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player constraint in physics>
use bevy::prelude::*;

use super::component::{Velocity};

use super::config::{PLAYER_CONTROL_SPEED_LIMIT, PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE};

use crate::event::{ForceEvent, PlayerIntentEvent, PlayerIntentKind};

pub(in crate::physics) fn player_intent_to_force_system(
    mut intent_events: EventReader<PlayerIntentEvent>,
    mut force_events: EventWriter<ForceEvent>,
    query: Query<&Velocity>,
) {
    let speed_limit = PLAYER_CONTROL_SPEED_LIMIT;
    let move_force = PLAYER_MOVE_FORCE;
    let jump_force = PLAYER_JUMP_FORCE;

    for event in intent_events.read() {
        let mut force_limit = Vec2::ZERO;
        let resistance_constant = move_force / speed_limit;
        match event.kind {
            PlayerIntentKind::Move { axis_x } => {
                if let Ok(velocity) = query.get(event.player) {
                    let speed = velocity.0.x * axis_x.signum();
                    if speed <= 0.0 {
                        force_limit.x = move_force * axis_x.signum();
                    } else if speed < speed_limit {
                        let resistance = resistance_constant * speed;
                        force_limit.x = (move_force - resistance) * axis_x.signum();
                    } else {
                        force_limit.x = 0.0;
                    }
                }
            }
            PlayerIntentKind::JumpStart => {
                force_limit.y = jump_force;
            }
        }
        force_events.write(ForceEvent {
            target: event.player,
            force: force_limit,
        })
    }

}