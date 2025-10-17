// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player mod>
use bevy::prelude::*;

mod spawn;

mod bundle;
mod config;
pub mod component;

use self::spawn::spawn_player;

use crate::event::RequestControl;
use crate::event::PlayerSpawnEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>()
        .add_event::<RequestControl>()
        .add_systems(Update, spawn_player);
    }
}
