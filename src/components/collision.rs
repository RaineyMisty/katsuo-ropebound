// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Collision components>

use bevy::prelude::*;
use bevy::math::Bounding::Aabb;

#[derive(Component)]
pub struct Collision { pub Aabb: Aabb }

impl Collision {
    pub fn from_size(size: Vec2, center: Vec2) -> Self {
        Collision { Aabb {
            center: Vec3::new(center.x, center.y, 0.0),
            half_extents: Vec3::new(size.x * 0.5, size.y * 0.5, 0.0),
        } }
    }

    pub fn intersects(&self, other: &Collision) -> bool {
        let a = &self.Aabb;
        let b = &other.Aabb;
        (a.center.x - b.center.x).abs() <= (a.half_extents.x + b.half_extents.x) &&
        (a.center.y - b.center.y).abs() <= (a.half_extents.y + b.half_extents.y)
    }
}