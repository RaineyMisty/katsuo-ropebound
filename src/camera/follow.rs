// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Xiaoting Wang
// Author: Xiaoting Wang <xiw323@pitt.edu>
// Description: <Camera Follow>
use bevy::prelude::*;

use super::resource::FollowTarget;
use crate::event::{Lifetime2CameraTarget};

pub(super) fn set_target(
    mut target: ResMut<FollowTarget>,
    mut event: EventReader::<Lifetime2CameraTarget>,
){
    for e in event.read() {
        target.0 = e.main_player;
    }
    println!("Camera target set to: {:?}", target.0);
}

pub(super) fn camera_follow(
    // time: Res<time>//
    // mut camera: Query<()>
){
}

