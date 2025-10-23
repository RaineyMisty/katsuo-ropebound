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
    mut register_events: EventWriter<RegisterRope>,
) {
    for event in events.read() {
        info!("Spawning rope between {:?} and {:?}", event.head_entity, event.tail_entity);
        let rest_length = ROPE_REST_LENGTH;
        let max_extension = ROPE_MAX_EXTENSION;
        let spring_constant = SPRING_CONSTANT;
        let rope_entity = commands.spawn((
            RopeBundle {
                spring_joint: SpringJoint {
                    rest_length,
                    max_extension,
                    spring_constant,
                },
                rope_ends: EndPoints {
                    head: EndPoint::Body(event.head_entity),
                    tail: EndPoint::Body(event.tail_entity),
                },
                transform: Transform::default(),
            },
            Rope,
        )).id();

        info!("Rope entity spawned: {:?}", rope_entity);
        register_events.write(RegisterRope {
            rope_entity,
            head_entity: event.head_entity,
            tail_entity: event.tail_entity,
            rest_length,
            max_extension,
            spring_constant,
        });
    }
}
