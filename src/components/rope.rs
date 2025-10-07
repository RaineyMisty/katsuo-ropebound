// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope component>
use bevy::prelude::*;
use crate::config::physics::{ROPE_REST_LENGTH, ROPE_MAX_EXTENSION, SPRING_CONSTANT};

#[derive(Clone, Copy, Debug)]
pub struct RopeConstraint {
    pub rest_length: f32,
    pub max_extension: f32,
    pub spring_constant: f32,
}

impl Default for RopeConstraint {
    fn default() -> Self {
        RopeConstraint {
            rest_length: ROPE_REST_LENGTH,
            max_extension: ROPE_MAX_EXTENSION,
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

/// 临时存放 rope 的几何信息
#[derive(Resource, Default)]
pub struct RopeGeometry {
    pub updates: Vec<(Entity, Vec3, f32, f32)>, // (rope_entity, 中点, 角度, 长度)
}

#[derive(Component)]
pub struct RopeSprite {
    pub rope_entity: Entity,
}