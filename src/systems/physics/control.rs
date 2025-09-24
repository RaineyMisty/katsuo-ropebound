// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*;
use crate::config::player::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::components::{Player, Velocity, Force};

fn player_movement_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Force, &Player), With<Player>>,
) {
    for (mut velocity, mut force, player) in &mut query {
        // Calculate the resistance parameter
        let resistance = PLAYER_CONTROL_SPEED_LIMIT / PLAYER_MOVE_FORCE;

        // Calculate the resistance force
        let resistance_force = resistance * velocity.velocity;

        // Horizontal force
        if keyboard_input.pressed(player.controls.left) {
            force.force.x -= PLAYER_MOVE_FORCE + resistance_force.x;
        }
        if keyboard_input.pressed(player.controls.right) {
            force.force.x += PLAYER_MOVE_FORCE - resistance_force.x;
        }

        // // Vertical force
        // if keyboard_input.pressed(player.controls.up) {
        //     force.force.y += PLAYER_MOVE_FORCE - resistance_force.y;
        // }
        // if keyboard_input.pressed(player.controls.down) {
        //     force.force.y -= PLAYER_MOVE_FORCE + resistance_force.y;
        // }
    }
}