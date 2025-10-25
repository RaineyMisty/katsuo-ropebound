// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Lifetime - Player Spawn>
use bevy::prelude::*;

use crate::event::{Player2LifetimeSpawned};
use super::component::SpawnTrack;

pub(super) fn wait_for_player_spawn(
    mut track: ResMut<SpawnTrack>,
    mut events: EventReader<Player2LifetimeSpawned>
) {
    for event in events.read() {
        let node = event.node as usize;
        if track.node_to_entity[node].is_none() {
            track.node_to_entity[node] = Some(event.entity);
            track.spawned_players += 1;
        }
    }
}