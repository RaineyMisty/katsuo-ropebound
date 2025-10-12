// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*;

use self::config::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE};
use self::component::{Player, ControlScheme, PlayerIntent};

use crate::event::{ForceEvent, ForceKind};

pub(super) fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&ControlScheme, &mut PlayerIntent), With<Player>>,
) {
    for (controls, mut intent) in query.iter_mut() {
        intent.move_left = keyboard_input.pressed(controls.left);
        intent.move_right = keyboard_input.pressed(controls.right);
        intent.jump = keyboard_input.just_pressed(controls.up);
    }
}

pub(super) fn player_control_system(
    mut event: EventWriter<ForceEvent>,
    mut query: Query<(Entity, &PlayerIntent), With<Player>>,
){
    for (entity, intent) in query.iter_mut() {
        let mut force = Vec2::ZERO;
        if intent.move_left {
            force.x -= PLAYER_MOVE_FORCE;
        }
        if intent.move_right {
            force.x += PLAYER_MOVE_FORCE;
        }
        if intent.jump {
            force.y += PLAYER_JUMP_FORCE;
        }
        if force != Vec2::ZERO {
            event.write(ForceEvent {
                target: entity,
                force,
                kind: ForceKind::PlayerPush { player: entity },
            });
        }
    }
}





//         // Calculate the resistance force (speed-dependent)
//         let resistance_force = resistance * velocity.0.x.abs();

//         // Reset control force
//         control_force.0 = Vec2::ZERO;

//         // Horizontal force
//         if keyboard_input.pressed(player.controls.left) {
//             if velocity.0.x > -PLAYER_CONTROL_SPEED_LIMIT { // speed up if not reached limit
//                 control_force.0.x = - PLAYER_MOVE_FORCE;
//                 if velocity.0.x < 0.0 {
//                     control_force.0.x += resistance_force;
//                 }
//             }
//         }
//         if keyboard_input.pressed(player.controls.right) {
//             if velocity.0.x < PLAYER_CONTROL_SPEED_LIMIT { // don't speed up if reached limit
//                 control_force.0.x = PLAYER_MOVE_FORCE;
//                 if velocity.0.x > 0.0 {
//                     control_force.0.x -= resistance_force;
//                 }
//             }
//         }

//         // Vertical force
//         if keyboard_input.just_pressed(player.controls.up) {
//             control_force.0.y = PLAYER_JUMP_FORCE;
//         }

//         net_force.0 += control_force.0;
//     }
// }