// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope component>
use bevy::prelude::*;
use crate::config::physics::{ROPE_MAX_LENGTH, SPRING_CONSTANT};

#[derive(Clone, Copy, Debug)]
pub struct RopeConstraint {
    pub rest_length: f32,
    pub max_length: f32,
    pub spring_constant: f32,
}

impl Default for RopeConstraint {
    fn default() -> Self {
        RopeConstraint {
            rest_length: ROPE_MAX_LENGTH * 0.5,
            max_length: ROPE_MAX_LENGTH,
            spring_constant: SPRING_CONSTANT,
        }
    }
}

#[derive(Component, Debug)]
pub struct Rope {
    pub constraint: RopeConstraint,
    pub attached_entity_head: Entity,
    pub attached_entity_tail: Entity,
}