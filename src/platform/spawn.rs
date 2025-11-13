// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Platform Spawn>
use bevy::prelude::*;

use crate::event::Mapload2PlatformSpawn;

#[derive(Component, Debug)]
pub struct Platform;

pub(super) fn platform_spawn(
    mut commands: Commands,
    mut events: EventReader<Mapload2PlatformSpawn>,
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
                    stretch_value: 1.0 * event.size.y / 64.0,
                },
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x, y, 1.0)),
        ));
    }
}