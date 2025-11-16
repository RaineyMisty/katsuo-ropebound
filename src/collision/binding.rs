// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Deal with Control Request>
use bevy::prelude::*;

use super::component::{Collider, Aabb2D, OnGround};
use crate::event::Entity2CollisionAttach;

fn clean_collision(
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity)
        .remove::<Collider>()
        .remove::<Aabb2D>()
        .remove::<OnGround>();
}

pub(super) fn on_request_collision(
    mut commands: Commands,
    mut reqs: EventReader<Entity2CollisionAttach>,
) {
    for req in reqs.read() {
        clean_collision(&mut commands, req.entity);
        commands.entity(req.entity).insert((
            Collider {
                extend: req.size,
            },
            Aabb2D {
                min: Vec2::ZERO,
                max: Vec2::ZERO,
            },
        ));
        if req.is_player == true {
            info!("attrched {}", req.entity);
            commands.entity(req.entity).insert(OnGround(false));
        }
    }
}