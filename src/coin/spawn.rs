// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Coin Spawn>
use bevy::prelude::*;

use crate::event::Mapload2CoinSpawn;

#[derive(Component, Debug)]
pub struct Coin;

pub(super) fn coin_spawn(
    mut commands: Commands,
    mut events: EventReader<Mapload2CoinSpawn>,
) {
    for event in events.read() {
        let x = event.position.x;
        let y = event.position.y;
        commands.spawn((
            Sprite {
                image: event.texture.clone(),
                custom_size: Some(event.size),
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 1.0,
                },
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x, y, 1.0)),
            Coin,
        ));
    }
}