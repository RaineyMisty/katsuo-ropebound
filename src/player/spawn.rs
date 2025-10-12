// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>

use bevy::prelude::*;

use crate::player::bundle::{PlayerBundle, PlayerControls};
use crate::player::config::PlayerSpawnPoint;
use crate::player::config::PlayerSpawnVelocity;
use crate::player::config::PLAYER_SPAWN_MASS;

use crate::physics::component::{Velocity, Mass};
// use crate::rope::component::{Rope};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, spawn_point: Res<PlayerSpawnPoint>, spawn_velocity: Res<PlayerSpawnVelocity>) {
    let transform = Transform::from_translation(spawn_point.position);
    let texture = asset_server.load("spriteguy.png");
    let controls = PlayerControls {
        up: KeyCode::KeyW,
        left: KeyCode::KeyA,
        right: KeyCode::KeyD,
    };
    commands.spawn(PlayerBundle::new(controls, texture, transform)).id();
}