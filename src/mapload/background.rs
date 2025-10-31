// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Backgound Spawn>
use bevy::prelude::*;

// TODO-Map Loading: Spawn background.

pub(super) fn load_background(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: assets.load("backgrounds/mount_bg.png"),
            ..Default::default()
        },
        Transform::from_xyz(640.0, 1024.0, -10.0),
    ));
}