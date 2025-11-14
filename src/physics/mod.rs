// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;
use bevy::time::Fixed;

mod physics_core;
mod player;
mod platform;
mod control;
mod collision;
mod rope;

mod config;
mod schedule;

use self::physics_core::PhysicsCorePlugin;
use self::player::player_insert_physics;
use self::platform::platform_insert_physics;
use self::rope::rope_insert_joint;
use self::rope::rope_tension_system;
use self::control::player_intent_to_force;
use self::collision::collision_info_to_impulse;
use self::schedule::PhysicsSet;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
           .configure_sets(
                FixedUpdate,
                (
                    PhysicsSet::Clear,
                    PhysicsSet::Emit,
                    PhysicsSet::Integrate,
                ).chain(),
            )
           .add_plugins(PhysicsCorePlugin)
           .add_systems(
                FixedUpdate,
                (
                    player_insert_physics,
                    platform_insert_physics,
                    rope_insert_joint,
                    player_intent_to_force,
                    rope_tension_system,
                    collision_info_to_impulse,
                ).in_set(PhysicsSet::Emit).chain()
            );
    }
}