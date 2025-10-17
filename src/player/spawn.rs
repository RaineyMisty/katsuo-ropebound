// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>
use bevy::prelude::*;

use super::component::Player;
use super::bundle::PlayerBundle;

use crate::physics::bundle::PhysicsBundle;
use crate::event::{PlayerSpawnEvent, RequestControl};

pub(super) fn spawn_player(
    mut commands: Commands,
    mut events: EventReader<PlayerSpawnEvent>,
    mut req_ctl: EventWriter<RequestControl>
) {
    for event in events.read() {
        let transform = Transform::from_translation(event.position.extend(0.0));
        let entity = commands.spawn((
            PlayerBundle::new(event.texture.clone(), transform),
            PhysicsBundle::new(event.mass.unwrap_or(1.0), true),
            Player,
        ))
        .id();

        req_ctl.write(RequestControl {
            entity,
            spec: event.controls.clone(),
        });
    }

}