// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Xiaoting Wang
// Author: Xiaoting Wang <XIW323@pitt.edu>
// Description: <Camera Spawn System>
use bevy::prelude::*;

use super::resource::SpawnTrack;

use crate::event::{Lifetime2CameraTarget};

pub(super) fn send_target(
    track: Res<SpawnTrack>,
    mut event: Eventwrite::<Lifetime2CameraTarget>,
){  
    let player = track.node_to_entity[0]; // assuming node 0 is the main player
    event.write(Lifetime2CameraTarget{
        main_player: player,
    })
}
