// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player Spawn System>
use bevy::prelude::*;

use crate::event::{PlayerSpawnEvent, ControlSpec};

pub(super) fn queue_for_player_setup_event(
    asset_server: Res<AssetServer>,
    mut events: EventWriter::<PlayerSpawnEvent>,
) {
    let tex: Handle<Image> = asset_server.load("portrait_rainey.png");
    events.write(PlayerSpawnEvent {
        texture: tex,
        position: Vec2::new(-100.0,-200.0),
        controls: ControlSpec::Keyboard {
            up: KeyCode::KeyW,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
        },
        mass: Some(50.0),
    });

    let tex: Handle<Image> = asset_server.load("portrait_shawn.png");
    events.write(PlayerSpawnEvent {
        texture: tex,
        position: Vec2::new(-300.0,-200.0),
        controls: ControlSpec::Keyboard {
            up: KeyCode::ArrowUp,
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
        },
        mass: Some(50.0),
    });

    let tex: Handle<Image> = asset_server.load("portrait_jagger.png");
    events.write(PlayerSpawnEvent {
        texture: tex,
        position: Vec2::new(-500.0,-200.0),
        controls: ControlSpec::Keyboard {
            up: KeyCode::KeyI,
            left: KeyCode::KeyJ,
            right: KeyCode::KeyL,
        },
        mass: Some(50.0),
    });
}