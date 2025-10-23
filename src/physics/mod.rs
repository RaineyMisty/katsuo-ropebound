// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics system module and plugin>
use bevy::prelude::*;

mod physics_core;
mod player;
// mod rope;

mod config;

use self::physics_core::PhysicsCorePlugin;
use self::player::player_intent_to_force_system;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsCorePlugin)
        .add_systems(Update, player_intent_to_force_system);
    }
}