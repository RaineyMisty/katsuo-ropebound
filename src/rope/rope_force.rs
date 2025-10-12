// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope force system>
use bevy::prelude::*;
use crate::components::rope::Rope;
use crate::physics::components::{RopeForce, NetForce};

pub fn clean_rope_force_system(mut q_rope_force: Query<&mut RopeForce>) {
    for mut rope_force in &mut q_rope_force {
        rope_force.0 = Vec2::ZERO;
    }
}

pub fn rope_tension_system(
    q_transforms: Query<&Transform>,
    mut q_rope_force: Query<&mut RopeForce>,
    q_rope: Query<&Rope>,
) {
    for rope in &q_rope {
        let Ok([head_transform, tail_transform]) =
            q_transforms.get_many([rope.attached_entity_head, rope.attached_entity_tail])
        else { continue; };

        let direction = (tail_transform.translation - head_transform.translation).truncate(); // to Vec2
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

        // Apply the force directly to the RopeForce components of both player entities
        if let Ok([mut head_rope_force, mut tail_rope_force]) = 
            q_rope_force.get_many_mut([rope.attached_entity_head, rope.attached_entity_tail])
        {
            head_rope_force.0 += force;
            tail_rope_force.0 -= force;
        }
    }
}

pub fn rope_force_to_system(
    mut query: Query<(&RopeForce, &mut NetForce)>,
) {
    for (rope_force, mut net_force) in &mut query {
        net_force.0 += rope_force.0;
    }
}