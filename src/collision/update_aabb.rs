// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Collision calculation>
use bevy::prelude::*;

use super::component::{Collider, Aabb2D};

pub(super) fn update_aabb (
    mut query: Query<(&Transform, &Collider, &mut Aabb2D)>,
) {
    for (tf, col, mut aabb) in query.iter_mut() {
        let center = tf.translation.truncate();
        let half = col.extend / 2.0;

        aabb.min = center - half;
        aabb.max = center + half;
    }
}