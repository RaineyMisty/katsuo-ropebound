// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope physics>
use bevy::prelude::*;

use super::physics_core::component::NetForce;

use crate::event::Rope2PhysicsAttach;
use crate::rope::component::{EndPoint, EndPoints};

pub(super) fn rope_insert_joint(
    mut commands: Commands,
    mut events: EventReader<Rope2PhysicsAttach>,
) {
    for event in events.read() {
        commands.entity(event.rope_entity).insert(
            SpringJoint {
                rest_length: event.rest_length,
                max_extension: event.max_extension,
                spring_constant: event.spring_constant,
            },
        );
    }
}

pub(super) fn rope_tension_system(
    q_transforms: Query<&GlobalTransform>,
    q_rope: Query<(&SpringJoint, &EndPoints)>, // used to have entity to mark the rope in ForceKind
    mut q_force: Query<&mut NetForce>
) {
    for (spring_joint, end_points) in &q_rope {
        let head_pos = match end_points.head {
            EndPoint::Body(e) => {
                if let Ok(t) = q_transforms.get(e) {
                    t.translation().truncate()
                } else {
                    continue; // skip if entity not found
                }
            },
            EndPoint::Fixed(p) => p,
        };

        let tail_pos = match end_points.tail {
            EndPoint::Body(e) => {
                if let Ok(t) = q_transforms.get(e) {
                    t.translation().truncate()
                } else {
                    continue; // skip if entity not found
                }
            },
            EndPoint::Fixed(p) => p,
        };

        let direction = tail_pos - head_pos; // Vec2
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

        if let EndPoint::Body(head_e) = end_points.head {
            if let Ok(mut net_force) = q_force.get_mut(head_e) {
                net_force.0 += force;
            }
        }
        if let EndPoint::Body(tail_e) = end_points.tail {
            if let Ok(mut net_force) = q_force.get_mut(tail_e) {
                net_force.0 -= force;
            }
        }
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub(super) struct SpringJoint {
    pub(super) rest_length: f32,
    pub(super) max_extension: f32,
    pub(super) spring_constant: f32,
}