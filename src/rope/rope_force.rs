// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope force system>
use bevy::prelude::*;
use super::components::{Rope};
use crate::event::{ForceEvent, ForceKind};

pub(super) fn rope_tension_system(
    mut events: EventWriter<ForceEvent>,
    q_transforms: Query<&GlobalTransform>,
    q_rope: Query<(Entity, &Rope)>,
) {
    for (entity, rope) in &q_rope {
        let Ok([head_transform, tail_transform]) =
            q_transforms.get_many([rope.attached_entity_head, rope.attached_entity_tail])
        else { continue; };

        let direction = (tail_transform.translation() - head_transform.translation()).truncate(); // to Vec2
        let current_length = direction.length();

        let force = if current_length > rope.constraint.rest_length {
            // F = -k * x
            let extension = current_length - rope.constraint.rest_length;
            let spring_constant = rope.constraint.spring_constant;
            let force_magnitude = spring_constant * extension;
            let force_direction = direction.normalize();
            force_direction * force_magnitude
        } else {
            Vec2::ZERO
        };

        // Write events to apply Rope Force
        events.send(ForceEvent {
            target: rope.attached_entity_head,
            force,
            kind: ForceKind::RopeTension { rope: entity },
        });
        events.send(ForceEvent {
            target: rope.attached_entity_tail,
            force: -force,
            kind: ForceKind::RopeTension { rope: entity },
        });
    }
}