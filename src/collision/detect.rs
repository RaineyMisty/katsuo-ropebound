// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Collision detect>
use bevy::prelude::*;

use super::component::{Collider, Aabb2D};
use crate::event::Collision2PhysicsInfo;

pub(super) fn collision_detect(
    query: Query<(Entity, &Aabb2D), With<Collider>>,
    mut event: EventWriter<Collision2PhysicsInfo>,
) {
    let entities: Vec<_> = query.iter().collect();
    for i in 0..entities.len() {
        let (ea, a) = entities[i];
        for j in (i + 1)..entities.len() {
            let (eb, b) = entities[j];

            if let Some((normal, penetration)) = aabb_vs_aabb(a, b) {
                event.write(Collision2PhysicsInfo {
                    entity_a: ea,
                    entity_b: eb,
                    normal,
                    penetration,
                });
            }
        }
    }
}

fn aabb_vs_aabb(a: &Aabb2D, b: &Aabb2D) -> Option<(Vec2, f32)> {
    if a.max.x < b.min.x || a.min.x > b.max.x ||
       a.max.y < b.min.y || a.min.y > b.max.y {
        return None;
    }

    let overlap_x1 = a.max.x - b.min.x;
    let overlap_x2 = b.max.x - a.min.x;
    let overlap_y1 = a.max.y - b.min.y;
    let overlap_y2 = b.max.y - a.min.y;
    
    let pen_x = overlap_x1.min(overlap_x2);
    let pen_y = overlap_y1.min(overlap_y2);

    if pen_x < pen_y {
        if a.max.x < b.max.x {
            Some((Vec2::new(-1.0, 0.0), pen_x))
        } else {
            Some((Vec2::new(1.0, 0.0), pen_x))
        }
    } else {
        if a.max.y < b.max.y {
            Some((Vec2::new(0.0, -1.0), pen_y))
        } else {
            Some((Vec2::new(0.0, 1.0), pen_y))
        }
    }
}