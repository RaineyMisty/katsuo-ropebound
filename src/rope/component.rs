// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope component>
use bevy::prelude::*;
use super::{ROPE_REST_LENGTH, ROPE_MAX_EXTENSION, SPRING_CONSTANT};

#[derive(Clone, Copy, Debug)]
struct RopeConstraint {
    rest_length: f32,
    _max_extension: f32,
    spring_constant: f32,
}

impl Default for RopeConstraint {
    fn default() -> Self {
        RopeConstraint {
            rest_length: ROPE_REST_LENGTH,
            _max_extension: ROPE_MAX_EXTENSION,
            spring_constant: SPRING_CONSTANT,
        }
    }
}

#[derive(Component, Debug)]
pub(super) struct Rope {
    pub(super) constraint: RopeConstraint,
    pub(super) attached_entity_head: Entity,
    pub(super) attached_entity_tail: Entity,
}