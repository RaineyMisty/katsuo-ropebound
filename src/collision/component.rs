// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Collision component>
use bevy::prelude::*;

#[derive(Component, Debug)]
pub(super) struct Collider {
    pub(super) extend: Vec2,
}

#[derive(Component, Debug)]
pub(super) struct Aabb2D {
    pub(super) min: Vec2,
    pub(super) max: Vec2,
}

#[derive(Component, Debug)]
pub struct OnGround (pub bool);