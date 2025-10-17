// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Key control method>
use bevy::prelude::*;

use super::component::{ControlScheme};

use crate::player::component::Player;
use crate::event::{PlayerIntentEvent, PlayerIntentKind};

pub(super) fn scheme_to_intent_writer (
    mut event: EventWriter<PlayerIntentEvent>,
    query: Query<(Entity, &ControlScheme), With<Player>>,
) {

    for (entity, control) in query.iter() {
        if control.move_axis != 0.0 {
            event.write(PlayerIntentEvent {
                player: entity,
                intent: PlayerIntentKind::Move { axis_x: control.move_axis },
            });
        }
        if control.jump_just {
            event.write(PlayerIntentEvent {
                player: entity,
                intent: PlayerIntentKind::JumpStart,
            });
        }
    }
}