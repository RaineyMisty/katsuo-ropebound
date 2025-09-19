// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::player::bundle::{PlayerBundle, PlayerControls};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("player.png");
    let controls = PlayerControls {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,
    };
    commands.spawn(PlayerBundle::new(controls, texture));
}