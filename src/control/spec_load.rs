// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Deal with Control Request>
use bevy::prelude::*;

use super::component::{KeyboardControlled, ControlScheme};
use crate::event::{RequestControl, ControlSpec};

fn clean_control(
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity)
        .remove::<ControlScheme>();
}

pub(super) fn on_request_control(
    mut commands: Commands,
    mut reqs: EventReader<RequestControl>,
) {
    for req in reqs.read() {
        clean_control(&mut commands, req.entity);
        match &req.spec {
            ControlSpec::Keyboard {up, left, right} => {
                commands.entity(req.entity).insert((
                    KeyboardControlled { up: *up, left: *left, right: *right },
                    ControlScheme::default(),
                ));
            }
            ControlSpec::Aibot {..} => {
                commands.entity(req.entity).insert((
                    ControlScheme::default(),
                ));
            }
        }
    }
}