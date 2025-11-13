// SPXD-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player plugin>
use bevy::prelude::*;

use super::component::Player;
use super::config::{PLAYER_SPAWN_MASS, PLAYER_SIZE};
use super::bundle::PlayerBundle;

use crate::event::Lifetime2PlayerSpawn;
use crate::event::Player2PhysicsAttach;
use crate::event::Player2ControlAttach;
use crate::event::Entity2CollisionAttach;
use crate::event::Player2LifetimeSpawned;

pub(super) fn spawn_player(
    mut commands: Commands,
    mut events: EventReader<Lifetime2PlayerSpawn>,
    mut req_phy: EventWriter<Player2PhysicsAttach>,
    mut req_ctl: EventWriter<Player2ControlAttach>,
    mut req_col: EventWriter<Entity2CollisionAttach>,
    mut spawned: EventWriter<Player2LifetimeSpawned>,
) {
    for event in events.read() {
        let transform = Transform::from_translation(event.position.extend(0.0));
        let entity = commands.spawn((
            PlayerBundle::new(event.texture.clone(), transform),
            Player,
        ))
        .id();

        let spawn_mass = PLAYER_SPAWN_MASS;
        req_phy.write(Player2PhysicsAttach {
            entity,
            mass: spawn_mass,
        });

        req_ctl.write(Player2ControlAttach {
            entity,
            spec: event.controls.clone(),
        });

        req_col.write(Entity2CollisionAttach {
            entity,
            size: PLAYER_SIZE,
        });

        spawned.write(Player2LifetimeSpawned {
            entity,
            node: event.node,
        });
    }

}