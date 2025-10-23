// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope Spawn system>
use bevy::prelude::*;

use crate::event::{RopeSpawnEvent, RegisterRope};

use super::component::{EndPoint, EndPoints, SpringJoint, Rope};
use super::config::{ROPE_REST_LENGTH, ROPE_MAX_EXTENSION, SPRING_CONSTANT};
use super::bundle::RopeBundle;

pub(super) fn spawn_rope(
    mut commands: Commands,
    mut events: EventReader<RopeSpawnEvent>,
) {
    for event in events.read() {
        info!("Spawning rope between {:?} and {:?}", event.head_entity, event.tail_entity);
        commands.spawn((
            RopeBundle {
                spring_joint: SpringJoint {
                    rest_length: ROPE_REST_LENGTH,
                    max_extension: ROPE_MAX_EXTENSION,
                    spring_constant: SPRING_CONSTANT,
                },
                rope_ends: EndPoints {
                    head: EndPoint::Body(event.head_entity),
                    tail: EndPoint::Body(event.tail_entity),
                },
                transform: Transform::default(),
            },
            Rope,
        ));


    }
}
