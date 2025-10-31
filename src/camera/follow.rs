// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Xiaoting Wang
// Author: Xiaoting Wang <xiw323@pitt.edu>
// Description: <Camera Follow>
use bevy::prelude::*;

use super::resource::FollowTarget;
use crate::event::{Lifetime2CameraTarget};
use crate::player::component::Player;

pub(super) fn set_target(
    mut target: ResMut<FollowTarget>,
    mut event: EventReader::<Lifetime2CameraTarget>,
){
    for e in event.read() {
        target.0 = e.main_player;
    }
}

pub(super) fn camera_follow(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
    target: Res<FollowTarget>,
    target_query: Query<&Transform, (With<Player>, Without<Camera>)>,
){
    if target.0.is_none() {
        return;
    }
    let target_entity = target.0.unwrap();
    let target_transform = match target_query.get(target_entity) {
        Ok(t) => t,
        Err(_) => return,
    };

    for mut cam_transform in camera.iter_mut() {
        let direction = target_transform.translation - cam_transform.translation;
        let distance = direction.length();
        if distance > 1.0 {
            let move_distance = distance * 5.0 * time.delta_secs();
            let movement = direction.normalize() * move_distance.min(distance);
            cam_transform.translation += movement;
        }
    }
}

