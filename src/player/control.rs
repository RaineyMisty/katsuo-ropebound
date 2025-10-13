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