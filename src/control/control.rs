// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Apply all control method>
use bevy::prelude::*;

use super::component::{Player, ControlScheme};

use crate::event::{PlayerIntentEvent, PlayerIntentKind};

pub(super) fn keyboard_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut ControlScheme, &KeyboardControlled), With<Player>>,
) {

    for (entity, controls, key) in query.iter_mut() {
        let mut left = keyboard_input.pressed(key.left);
        let mut right = keyboard_input.pressed(key.right);
        let jump_just = keyboard_input.just_pressed(key.up);
        let mut axis_x = 0.0;
        if left {
            axis_x -= 1.0;
        }
        if right {
            axis_x += 1.0;
        }
        control.move_axis = axis_x;
        control.jump_just = jump_just;
    }
}