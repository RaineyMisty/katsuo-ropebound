// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope force system>
use bevy::prelude::*;
use super::component::{SpringJoint, Endpoints};
use crate::event::{ForceEvent};

pub(super) fn rope_tension_system(
    mut events: EventWriter<ForceEvent>,
    q_transforms: Query<&GlobalTransform>,
    q_rope: Query<(&SpringJoint, &Endpoints)> // used to have entity to mark the rope in ForceKind
) {
    for (spring_joint, rope_ends) in &q_rope {
        let Ok([head_transform, tail_transform]) =
            q_transforms.get_many([rope_ends.head, rope_ends.tail])
        else { continue; };

        let direction = (tail_transform.translation() - head_transform.translation()).truncate(); // to Vec2
        let current_length = direction.length();

        let force = if current_length > spring_joint.rest_length {
            // F = -k * x
            let extension = current_length - spring_joint.rest_length;
            let spring_constant = spring_joint.spring_constant;
            let force_magnitude = spring_constant * extension;
            let force_direction = direction.normalize();
            force_direction * force_magnitude
        } else {
            Vec2::ZERO
        };

        // Write events to apply Rope Force
        events.write(ForceEvent {
            target: rope_ends.head,
            force,
        });
        events.write(ForceEvent {
            target: rope_ends.tail,
            force: -force,
        });
    }
}