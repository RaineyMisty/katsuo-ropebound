// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Tingxu Chen
// Description: <Map system>
use bevy::prelude::*;

mod background;
mod resource;
mod level_load;

use self::background::load_background;
use self::level_load::load_level_from_ron;

pub struct MapLoadPlugin;

impl Plugin for MapLoadPlugin {
    fn build(&self, app: &mut App) {
        app//.init_resource::<LevelRes>()
        .add_systems(Startup, load_level_from_ron)
        .add_systems(Startup, load_background);
    }
}
