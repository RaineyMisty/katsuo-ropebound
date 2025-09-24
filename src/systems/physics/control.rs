// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*;
use crate::config::player::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::bundle::Player;
use crate::components::motion::{Velocity, Force};

fn player_movement_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Force, &Player), With<Player>>,
) {
    for (mut velocity, mut force, player) in &mut query {
        // Calculate the resistance parameter
        let resistance = PLAYER_CONTROL_SPEED_LIMIT / PLAYER_MOVE_FORCE;

        // Calculate the resistance force
        let resistance_force = resistance * velocity.velocity;

        // Apply resistance force
        force.0.x -= resistance_force.x;
        force.0.y -= resistance_force.y;

        // Horizontal force
        if keyboard_input.pressed(player.controls.left) {
            force.0.x -= PLAYER_MOVE_FORCE;
        }
        if keyboard_input.pressed(player.controls.right) {
            force.0.x += PLAYER_MOVE_FORCE;
        }

        // // Vertical force
        // if keyboard_input.pressed(player.controls.up) {
        //     force.0.y += PLAYER_MOVE_FORCE - resistance_force.y;
        // }
        // if keyboard_input.pressed(player.controls.down) {
        //     force.0.y -= PLAYER_MOVE_FORCE + resistance_force.y;
        // }
    }
}