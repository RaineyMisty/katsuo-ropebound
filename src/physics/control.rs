// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*;
use crate::config::player::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::bundle::Player;
use crate::components::motion::{Velocity, Force};

pub fn player_movement_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Force, &Player), With<Player>>,
) {
    for (mut velocity, mut force, player) in &mut query {
        // Calculate the resistance parameter
        // f = c*v => c = f/v
        let resistance = PLAYER_MOVE_FORCE / PLAYER_CONTROL_SPEED_LIMIT; 
        let resistance_y = PLAYER_JUMP_FORCE / PLAYER_CONTROL_SPEED_LIMIT;

        // Calculate the resistance force (speed-dependent)
        let resistance_force = resistance * velocity.0;
        let resistance_force_y = resistance_y * velocity.0;

        // Reset force
        force.0 = Vec2::ZERO;

        // Horizontal force
        if keyboard_input.pressed(player.controls.left) {
            force.0.x = - PLAYER_MOVE_FORCE;
            if velocity.0.x < 0.0 {
                force.0.x += resistance_force.x.abs();
            }
        }
        if keyboard_input.pressed(player.controls.right) {
            force.0.x = PLAYER_MOVE_FORCE;
            if velocity.0.x > 0.0 {
                force.0.x -= resistance_force.x.abs();
            }
        }

        // Vertical force
        if keyboard_input.pressed(player.controls.up) {
            force.0.y = PLAYER_JUMP_FORCE;
            if velocity.0.y > 0.0 {
                force.0.y -= resistance_force_y.y.abs();
            }
        }
        if keyboard_input.pressed(player.controls.down) {
            force.0.y = - PLAYER_JUMP_FORCE;
            if velocity.0.y < 0.0 {
                force.0.y += resistance_force_y.y.abs();
            }
        }
    }
}