// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Lifetime System>
use bevy::prelude::*;

mod player_spawn;
mod rope_spawn;
mod camera_spawn;
mod spawn_track;

mod resource;

use self::player_spawn::player_spawn;
use self::camera_spawn::camera_spawn;
use self::spawn_track::player_spawn_track;
use self::rope_spawn::rope_spawn;
use self::resource::SpawnTrack;

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin{
    fn build (&self, app: &mut App){
        app.init_resource::<SpawnTrack>()
        .add_plugins((
            crate::control::ControlPlugin,
            crate::player::PlayerPlugin,
            crate::rope::RopePlugin
        ))
        .add_systems(Startup, player_spawn)
        .add_systems(Update, player_spawn_track)
        .add_systems(Update, camera_spawn)
        .add_systems(Update, rope_spawn);
    }
}