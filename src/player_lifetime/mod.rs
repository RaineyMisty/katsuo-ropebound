// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Lifetime System>
use bevy::prelude::*;

mod player_spawn;

use self::player_lifetime::queue_for_player_setup_event;

pub struct PlayerLifetimePlugin;

impl Plugin for PlayerLifetimePlugin{
    fn build (&self app: mut App){
        app.add_event<PlayerSpawnEvent>()
        .add_systems(Startup, queue_for_player_setup_event);
    }
}