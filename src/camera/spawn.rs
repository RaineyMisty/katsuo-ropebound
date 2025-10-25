// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Xiaoting Wang
// Author: Xiaoting Wang <xiw323@pitt.edu>
// Description: <Camera Spawn>
use bevy::prelude::*;

pub(super) fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera::default(),
    ));
}
