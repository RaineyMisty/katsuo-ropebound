// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Camera mod>
use bevy::prelude::*;

mod follow;
mod spawn;
mod resource;

use self::spawn::setup_camera;
use self::follow::set_target;
use self::follow::camera_follow;
use self::resource::FollowTarget;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FollowTarget>()
           .add_systems(Startup, setup_camera)
           .add_systems(Update, set_target)
           .add_systems(Update, camera_follow);

    }
}