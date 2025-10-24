// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>
use bevy::prelude::*;

use super::component::Player;
use super::config::PLAYER_SPAWN_MASS;
use super::bundle::PlayerBundle;

use crate::event::{PlayerSpawnEvent, RequestPlayerPhysics, RequestControl, PlayerSpawned};

pub(super) fn spawn_player(
    mut commands: Commands,
    mut events: EventReader<PlayerSpawnEvent>,
    mut req_phy: EventWriter<RequestPlayerPhysics>,
    mut req_ctl: EventWriter<RequestControl>,
    mut spawned: EventWriter<PlayerSpawned>,
) {
    for event in events.read() {
        let transform = Transform::from_translation(event.position.extend(0.0));
        let entity = commands.spawn((
            PlayerBundle::new(event.texture.clone(), transform),
            Player,
        ))
        .id();

        let spawn_mass = PLAYER_SPAWN_MASS;
        req_phy.write(RequestPlayerPhysics {
            entity,
            mass: spawn_mass,
        });

        req_ctl.write(RequestControl {
            entity,
            spec: event.controls.clone(),
        });

        spawned.write(PlayerSpawned {
            entity,
            node: event.node,
        });
    }

}