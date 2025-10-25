// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Camera mod>
use bevy::prelude::*;

// mod follow;
mod spawn;

use self::spawn::setup_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        //    .app_systems(Update, camera_follow_2d);

    }
}