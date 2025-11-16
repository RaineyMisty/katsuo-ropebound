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
        // info!("Entity {} and {} collide, on {} and {}.", ea, eb, normal, penetration);
        if let Ok([(velocity_a, rb_a, mut impulse_a),
                (velocity_b, rb_b, mut impulse_b)])
                = query.get_many_mut([ea, eb])
        {
            let inv_mass_a = rb_a.inv_mass;
            let inv_mass_b = rb_b.inv_mass;
            let inv_mass_sum = inv_mass_a + inv_mass_b;
            if inv_mass_sum == 0.0 {
                continue;
            }

            let relevent_velocity = velocity_a.0 - velocity_b.0; // If the positions of a and b are written backwards, the player's momentum will continuously increase.
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

            if inv_mass_a > 0.0 {
                impulse_a.0 += impulse_total;
            }
            if inv_mass_b > 0.0 {
                impulse_b.0 -= impulse_total;
            }

            // info!("impulse to a is {}, impulse to b is {}", -impulse_total, impulse_total);

        } else {
            continue;
        }
    }
}

pub(super) fn resolve_penetration (
    mut events: EventReader<Collision2PhysicsInfo>,
    mut query: Query<(&mut Transform, &RigidBody)>,
) {
    const PERCENT: f32 = 1.0;
    const SLOP: f32 = 0.0001;

    for event in events.read() {
        let ea = event.entity_a;
        let eb = event.entity_b;
        if ea == eb {
            continue;
        }
        let mut normal = event.normal;
        if normal.length_squared() == 0.0 {
            continue;
        }
        normal = normal.normalize();
        let mut penetration = event.penetration;
        if penetration <= SLOP {
            continue;
        }

        penetration -= SLOP;
        if let Ok([(mut tf_a, rb_a),
                   (mut tf_b, rb_b)])
                   = query.get_many_mut([ea, eb])
        {
            let inv_mass_a = rb_a.inv_mass;
            let inv_mass_b = rb_b.inv_mass;
            let inv_mass_sum = inv_mass_a + inv_mass_b;

            if inv_mass_sum == 0.0 {
                continue;
            }

            let correction_mag = penetration * PERCENT;
            let correction = normal * correction_mag;

            let factor_a = inv_mass_a / inv_mass_sum;
            let factor_b = inv_mass_b / inv_mass_sum;

            let move_a = correction * factor_a;
            let move_b = -correction * factor_b;

            tf_a.translation.x += move_a.x;
            tf_a.translation.y += move_a.y;
            tf_b.translation.x += move_b.x;
            tf_b.translation.y += move_b.y;
        }
    }
}