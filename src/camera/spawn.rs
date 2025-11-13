// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Xiaoting Wang
// Author: Xiaoting Wang <xiw323@pitt.edu>
// Description: <Camera Spawn>
use bevy::prelude::*;

pub(super) fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera::default(),
        Camera2d,
        Transform::from_xyz(640.0, 360.0, 1000.0),
    ));
}
