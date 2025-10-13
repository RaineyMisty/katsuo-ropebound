// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*;

use self::component::{Player, ControlScheme};

use crate::event::{PlayerIntentEvent, PlayerIntentKind};

pub(super) fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<PlayerIntentEvent>,
    mut query: Query<(Entity, &ControlScheme), With<Player>>,
) {
    for (entity, controls) in query.iter_mut() {
        let mut axis_x = 0.0;
        if keyboard_input.pressed(controls.left){
            axis_x -= 1.0;
        }
        if keyboard_input.pressed(controls.right) {
            axis_x += 1.0;
        }
        if axis_x != 0.0 {
            event.send(PlayerIntentEvent {
                target: entity,
                kind: PlayerIntentKind::Move { direction: axis_x },
            });
        }
        if keyboard_input.just_pressed(controls.up) {
            event.send(PlayerIntentEvent {
                target: entity,
                kind: PlayerIntentKind::JumpStart,
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