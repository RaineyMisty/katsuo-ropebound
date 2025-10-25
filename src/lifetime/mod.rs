// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Lifetime System>
use bevy::prelude::*;

mod player_spawn;
mod rope_spawn;
mod camera_spawn;

mod resource;

use self::player_spawn::queue_for_player_setup_event;
use self::camera_spawn::send_target;
use self::rope_spawn::wait_for_player_spawn;
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
        .add_systems(Startup, queue_for_player_setup_event)
        .add_systems(Update, send_target)
        .add_systems(Update, wait_for_player_spawn);
    }
}