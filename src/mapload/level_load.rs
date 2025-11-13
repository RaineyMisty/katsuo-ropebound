// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Tingxu Chen
// Description: <load level>
use bevy::prelude::*;
use std::fs;

use super::resource::{LevelRes, Level};

pub(super) fn load_level_from_ron(mut commands: Commands) {
    let path = "assets/levels/level1.ron";

    let ron_str = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path));

    let level: Level = ron::from_str(&ron_str)
        .unwrap_or_else(|e| panic!("Failed to parse RON {}: {e}", path));

    info!(
        "Loaded level from RON: version={}, objects={}",
        level.version,
        level.objects.len()
    );

    commands.insert_resource(LevelRes(level));
}