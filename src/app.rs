// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <you@example.com>
// Description: <Create App and setup camera>

use bevy::prelude::*;
use bevy::time::Fixed;

mod player;
use player::PlayerPlugin;

pub fn run() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera::default(),
    ));
}
