// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Collision mod>
use bevy::prelude::*;

mod binding;
mod update_aabb;
mod detect;
mod ground;

pub mod component;

use self::binding::on_request_collision;
use self::update_aabb::update_aabb;
use self::detect::collision_detect;
use self::ground::{on_ground_init, on_ground_detect};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            on_request_collision,
            update_aabb,
            collision_detect,
            on_ground_init,
            on_ground_detect,
        ).chain());
    }
}