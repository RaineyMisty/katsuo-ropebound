// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope Move>
use bevy::prelude::*;

use super::component::{EndPoints, EndPoint, Rope};

pub(super) fn move_rope(
    mut ropes: Query<(&EndPoints, &mut Transform), With<Rope>>,
    world_tfs: Query<&GlobalTransform>,
) {
    for (ends, mut tf) in ropes.iter_mut() {
        let EndPoint::Body(head_e) = ends.head else { continue };
        let EndPoint::Body(tail_e) = ends.tail else { continue };

        let Ok([head_gt, tail_gt]) = world_tfs.get_many([head_e, tail_e]) else {
            continue;
        };

        let head = head_gt.translation();
        let tail = tail_gt.translation();

        let mid = (head + tail) * 0.5;
        tf.translation.x = mid.x;
        tf.translation.y = mid.y;
    }
}
