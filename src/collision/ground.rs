// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Tingxu Chen
// Description: <On Ground detect>
use bevy::prelude::*;

use super::component::{OnGround};
use crate::player::component::Player;
use crate::event::Collision2PhysicsInfo;

pub(super) fn on_ground_init(
    mut q_player: Query<&mut OnGround, With<Player>>
){
    for mut on_ground in q_player.iter_mut() {
        on_ground.0 = false;
    }
}

pub(super) fn on_ground_detect(
    mut q_player: Query<&mut OnGround, With<Player>>,
    mut events: EventReader<Collision2PhysicsInfo>,
) {
    for event in events.read() {
        if let Ok(mut ground) = q_player.get_mut(event.entity_a){
            info!("get a");
            if event.normal.y == -1.0 {
                ground.0 = true;
            }
        }
        if let Ok(mut ground) = q_player.get_mut(event.entity_b){
            if event.normal.y == 1.0 {
                ground.0 = true;
            }
        }
    }
}