// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>
use bevy::prelude::*;

use self::component::Player;
use self::bundle::PlayerBundle;
use self::event::PlayerSpawnEvent;

use crate::physics::bundle::PhysicsBundle;

pub(super) fn spawn_player(
    mut commands: Commands,
    mut events: EventReader<PlayerSpawnEvent>,
) {
    for event in events.iter() {
        let transform = Transform::from_translation(event.position.extend(0.0));
        commands.spawn((
            PlayerBundle::new(event.controls.clone(), event.texture.clone(), transform),
            PhysicsBundle::new(event.mass.unwrap_or(1.0), true),
            Player {
                name: event.name.clone(),
            },
        ));
    }
}