// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Tingxu Chen
// Description: <Map system>
use bevy::prelude::*;

mod background;
mod resource;
mod level_load;
mod debug;
mod map_spawn;

use self::background::load_background;
use self::level_load::load_level_from_ron;
use self::debug::spawn_level_labels;
use self::debug::draw_level_gizmos;
use self::map_spawn::map_spawn;
use self::resource::LevelRes;

pub struct MapLoadPlugin;

impl Plugin for MapLoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelRes>()
        .add_plugins((
            crate::platform::PlatformPlugin,
            crate::coin::CoinPlugin,
        ))
        .add_systems(Startup, load_level_from_ron)
        .add_systems(Startup, (
            load_background,
            map_spawn
        ).chain())
        .add_systems(Update, draw_level_gizmos)
        .add_systems(Update, spawn_level_labels);
    }
}
