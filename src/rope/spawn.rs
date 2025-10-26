// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope Spawn system>
use bevy::prelude::*;

use crate::event::{Lifetime2RopeSpawn, Rope2PhysicsAttach};

use super::component::{EndPoint, EndPoints, SpringJoint, Rope};
use super::config::{ROPE_REST_LENGTH, ROPE_MAX_EXTENSION, SPRING_CONSTANT};
use super::bundle::RopeBundle;

pub(super) fn spawn_rope(
    mut commands: Commands,
    mut events: EventReader<Lifetime2RopeSpawn>,
    mut register_events: EventWriter<Rope2PhysicsAttach>,
    tf: Query<&GlobalTransform>,
) {
    let rest_length = ROPE_REST_LENGTH;
    let max_extension = ROPE_MAX_EXTENSION;
    let spring_constant = SPRING_CONSTANT;
    for event in events.read() {
        let Ok([head_gt, tail_gt]) = tf.get_many([event.head_entity, event.tail_entity]) else {
            continue;
        };
        let head_pos = head_gt.translation();
        let tail_pos = tail_gt.translation();
        let mid = (head_pos + tail_pos) * 0.5;
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
                transform: Transform::from_translation(mid),
            },
            Rope,
        )).id();

        register_events.write(Rope2PhysicsAttach {
            rope_entity,
            head_entity: event.head_entity,
            tail_entity: event.tail_entity,
            rest_length,
            max_extension,
            spring_constant,
        });
    }
}
