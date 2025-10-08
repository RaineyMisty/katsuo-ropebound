// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>

use bevy::prelude::*;

use crate::config::PLAYER_SPAWN_MASS;
use crate::config::PlayerSpawnPoint;
use crate::config::PlayerSpawnVelocity;
use crate::player::bundle::{PlayerBundle, PlayerControls};

use crate::components::motion::{GroundState, JumpController, Mass, Velocity};
use crate::components::rope::{Rope, RopeConstraint};

use crate::app::{MainPlayer};


pub fn spawn_players(
    mut commands: Commands, #[cfg(feature = "client")] asset_server: Res<AssetServer>,
    spawn_point: Res<PlayerSpawnPoint>,
    spawn_velocity: Res<PlayerSpawnVelocity>,
) {
    // --- Spawn first player ---
    let p1 = spawn_single_player(
        &mut commands,
        #[cfg(feature = "client")]
        &asset_server,
        Transform::from_translation(spawn_point.position),
        spawn_velocity.velocity,
        PlayerControls {
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
        },
        #[cfg(feature = "client")]
        "spriteguy.png",
        true,
    );

    // --- Spawn second player (test) ---
    let p2 = spawn_single_player(
        &mut commands,
        #[cfg(feature = "client")]
        &asset_server,
        Transform::from_translation(spawn_point.position + Vec3::new(300.0, 0.0, 0.0)),
        spawn_velocity.velocity,
        PlayerControls {
            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
        },
        #[cfg(feature = "client")]
        "portrait_rainey.png",
        false, // mark as MainPlayer
    );

    // Rope between the two players
    commands.spawn(Rope {
        constraint: RopeConstraint::default(),
        attached_entity_head: p1,
        attached_entity_tail: p2,
    });
}

fn spawn_single_player(
    commands: &mut Commands,
    #[cfg(feature = "client")] asset_server: &AssetServer,
    transform: Transform,
    velocity: Vec2,
    controls: PlayerControls,
    #[cfg(feature = "client")] texture_path: &str,
    is_main: bool,
) -> Entity {
    let jump_controller = JumpController::default();
    let ground_state = GroundState::default();
    let mass = Mass(PLAYER_SPAWN_MASS);
    let velocity = Velocity(velocity);

    let mut entity_commands = commands.spawn(PlayerBundle::new(
        controls,
        #[cfg(feature = "client")]
        asset_server.load(texture_path),
        transform,
        velocity,
        mass,
        jump_controller,
        ground_state,
    ));

    if is_main {
        entity_commands.insert(MainPlayer);
    }

    entity_commands.id()
}
