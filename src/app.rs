// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use bevy::prelude::*;
use bevy::time::Fixed;
use crate::control::ControlPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::player_lifetime::PlayerLifetimePlugin;
use crate::rope::RopePlugin;
use crate::event::{ForceEvent, ImpulseEvent, PlayerIntentEvent};

pub fn run() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "katsuo_ropebound=debug,bevy=warn,wgpu=warn,naga=warn".to_string(),
            ..Default::default()
        }))
        .add_event::<ForceEvent>()
        .add_event::<ImpulseEvent>()
        .add_event::<PlayerIntentEvent>()
        .add_event::<PlayerSpawnEvent>()
        .add_plugins(PlayerLifetimePlugin)
        .add_systems(ControlPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(RopePlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera::default(),
    ));
}
