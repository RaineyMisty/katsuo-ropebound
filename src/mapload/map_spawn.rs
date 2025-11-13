// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Tingxu Chen
// Description: <send map spawn info>
use bevy::prelude::*;

use super::resource::{LevelRes, Kind};
use crate::event::Mapload2PlatformSpawn;
use crate::event::Mapload2CoinSpawn;

pub(super) fn map_spawn(
    asset_server: Res<AssetServer>,
    level_res: Res<LevelRes>,
    mut platform_event: EventWriter<Mapload2PlatformSpawn>,
    mut coin_event: EventWriter<Mapload2CoinSpawn>,
) {
    let level = &level_res.0;

    let platform_rock: Handle<Image> = asset_server.load("tiles/platform_rock.png");
    let coin: Handle<Image> = asset_server.load("tiles/coin.png");

    for obj in level.objects.iter() {
        match obj.kind {
            Kind::Platform => {
                platform_event.write(Mapload2PlatformSpawn {
                    texture: platform_rock.clone(),
                    position: obj.pos,
                    size: obj.size,
                });
            },
            Kind::Coin => {
                coin_event.write(Mapload2CoinSpawn {
                    texture: coin.clone(),
                    position: obj.pos,
                    size: obj.size,
                });
            },
            _ => {}
        }
    }
}