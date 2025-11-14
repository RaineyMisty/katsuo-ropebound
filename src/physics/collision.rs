// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Collision physics>
use bevy::prelude::*;

use super::physics_core::component::{Velocity, Impulse, RigidBody};

use crate::event::Collision2PhysicsInfo;

pub(super) fn collision_info_to_impulse (
    mut events: EventReader<Collision2PhysicsInfo>,
    mut query: Query<(&Velocity, &RigidBody, &mut Impulse)>,
) {
    for event in events.read() {
        let ea = event.entity_a;
        let eb = event.entity_b;
        if ea == eb {
            continue;
        }
        let normal = event.normal;
        let penetration = event.penetration;
        info!("Entity {} and {} collide, on {} and {}.", ea, eb, normal, penetration);
        if let Ok([(velocity_a, rb_a, mut impulse_a),
                (velocity_b, rb_b, mut impulse_b)])
                = query.get_many_mut([ea, eb])
        {
            let inv_mass_sum = rb_a.inv_mass + rb_b.inv_mass;
            if inv_mass_sum == 0.0 {
                continue;
            }

            let relevent_velocity = velocity_b.0 - velocity_a.0; // If the positions of a and b are written backwards, the player's momentum will continuously increase.
            let velocity_alone_normal = relevent_velocity.dot(normal);
            if velocity_alone_normal > 0.0 {
                continue;
            }

            let e = rb_a.restitution.min(rb_b.restitution);
            let j_n = -(1.0 + e) * velocity_alone_normal / inv_mass_sum;
            let impulse_normal = j_n * normal;

            let tangent_raw = relevent_velocity - velocity_alone_normal * normal;
            let mut impulse_tangent = Vec2::ZERO;
            if tangent_raw.length_squared() > 0.0 {
                let tangent = tangent_raw.normalize();
                let velocity_alone_tangent = relevent_velocity.dot(tangent);
                let j_t_ideal = -velocity_alone_tangent / inv_mass_sum;
                let mu = rb_a.friction.min(rb_b.friction);
                let max_j_t = mu  * j_n.abs();
                let j_t = j_t_ideal.clamp(-max_j_t, max_j_t);
                impulse_tangent = j_t * tangent;
            }

            let impulse_total = impulse_normal + impulse_tangent;

            if rb_a.inv_mass > 0.0 {
                impulse_a.0 -= impulse_total;
            }
            if rb_b.inv_mass > 0.0 {
                impulse_b.0 += impulse_total;
            }

        } else {
            continue;
        }
    }
}