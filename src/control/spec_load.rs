// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Deal with Control Request>
use bevy::prelude::*;

use super::component::KeyboardControlled;
use crate::event::{RequestControl, ControlSpec};

fn clean_control(
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity)
        .remove::<ControlScheme>();
}

pub(in crate::control) fn on_request_control(
    mut commands: Commands,
    mut reqs: EventReader<RequestControl>,
) {
    for req in reqs.read() {
        clean_control(&commands, req.entity);
        match &req.spec {
            ControlSpec::Keyboard {up, left, right} => {
                commands.entity(entity).insert((
                    KeyboardControlled { up: *up, left: *left, right: *right },
                    ControlScheme,
                ));
            }
            ControlSpec::Aibot {..} => {
                commands.entity(entity).insert((
                    ControlScheme,
                ));
            }
        }
    }
}