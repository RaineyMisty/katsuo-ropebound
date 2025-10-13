// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player mod>
use bevy::prelude::*;

mod control;
mod spawn;

mod bundle;
mod config;
mod component;
mod event;

use self::event::PlayerSpawnEvent;
use self::spawn::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>()
        .add_systems(Startup, queue_for_player_setup_event)
        .add_systems(Update, spawn_player);
    }
}

use self::component::ControlScheme;

fn queue_for_player_setup_event(
    asset_server: Res<AssetServer>,
    mut events: EventWriter::<PlayerSpawnEvent>,
) {
    let tex: Handle<Image> = asset_server.load("portrait_rainey.png");
    events.write(PlayerSpawnEvent {
        texture: tex,
        position: Vec2::new(-500.0,-200.0),
        controls: ControlScheme{
            up: KeyCode::KeyW,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
        },
        mass: Some(50.0),
    });
}