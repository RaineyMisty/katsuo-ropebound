// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Tingxu Chen
// Description: <send map spawn info>
use bevy::prelude::*;

use super::resource::{LevelRes, Kind};
use crate::event::Mapload2PlatformSpawn;

pub(super) fn map_spawn(
    asset_server: Res<AssetServer>,
    level_res: Res<LevelRes>,
    mut event: EventWriter<Mapload2PlatformSpawn>,
) {
    let level = &level_res.0;

    let tex: Handle<Image> = asset_server.load("tiles/platform_rock.png");

    for obj in level.objects.iter() {
        match obj.kind {
            Kind::Platform => {
                event.write(Mapload2PlatformSpawn{
                    texture: tex.clone(),
                    position: obj.pos,
                    size: obj.size,
                });
            },
            _ => {}
        }
    }
}