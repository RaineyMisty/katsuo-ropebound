// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*;
use crate::config::physics::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::bundle::Player;
use crate::components::motion::{Velocity, ControlForce, NetForce};

pub fn player_movement_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut ControlForce, &mut NetForce, &Player), With<Player>>,
) {
    for (mut velocity, mut control_force, mut net_force, player) in &mut query {
        // Calculate the resistance parameter
        // f = c*v => c = f/v
        let resistance = PLAYER_MOVE_FORCE / PLAYER_CONTROL_SPEED_LIMIT; 

        // Calculate the resistance force (speed-dependent)
        let resistance_force = resistance * velocity.0.x.abs();

        // Reset control force
        control_force.0 = Vec2::ZERO;

        // Horizontal force
        if keyboard_input.pressed(player.controls.left) {
            if velocity.0.x > -PLAYER_CONTROL_SPEED_LIMIT { // speed up if not reached limit
                control_force.0.x = - PLAYER_MOVE_FORCE;
                if velocity.0.x < 0.0 {
                    control_force.0.x += resistance_force;
                }
            }
        }
        if keyboard_input.pressed(player.controls.right) {
            if velocity.0.x < PLAYER_CONTROL_SPEED_LIMIT { // don't speed up if reached limit
                control_force.0.x = PLAYER_MOVE_FORCE;
                if velocity.0.x > 0.0 {
                    control_force.0.x -= resistance_force;
                }
            }
        }

        // Vertical force
        if keyboard_input.just_pressed(player.controls.up) {
            control_force.0.y = PLAYER_JUMP_FORCE;
        }

        net_force.0 += control_force.0;
    }
}