// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Lifetime System>
use bevy::prelude::*;

mod player_spawn;
mod rope_spawn;

mod component;

use self::player_spawn::queue_for_player_setup_event;
use self::rope_spawn::wait_for_player_spawn;
use self::component::SpawnTrack;

use crate::event::PlayerSpawnEvent;

pub struct PlayerLifetimePlugin;

impl Plugin for PlayerLifetimePlugin{
    fn build (&self, app: &mut App){
        app.init_resource::<SpawnTrack>()
        .add_event::<PlayerSpawnEvent>()
        .add_systems(Startup, queue_for_player_setup_event)
        .add_systems(Update, wait_for_player_spawn);
    }
}