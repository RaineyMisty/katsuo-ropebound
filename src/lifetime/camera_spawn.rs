// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Xiaoting Wang
// Author: Xiaoting Wang <XIW323@pitt.edu>
// Description: <Camera Spawn System>
use bevy::prelude::*;

use super::resource::SpawnTrack;

use crate::event::{Lifetime2CameraTarget};

pub(super) fn camera_spawn(
    track: Res<SpawnTrack>,
    mut event: EventWriter::<Lifetime2CameraTarget>,
){  
    if track.main_player.is_none() {
        event.write(Lifetime2CameraTarget {
            main_player: None,
        });
        return;
    }
    let main_index = track.main_player.unwrap();
    let player = track.node_to_entity[main_index];
    event.write(Lifetime2CameraTarget {
        main_player: player,
    });
}
