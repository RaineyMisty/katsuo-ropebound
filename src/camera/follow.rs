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
    mut camera: Query<(&mut Transform, &Projection), With<Camera>>,
    target: Res<FollowTarget>,
    target_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let Some(target_entity) = target.0 else {
        return;
    };

    let target_transform = match target_query.get(target_entity) {
        Ok(t) => t,
        Err(_) => return,
    };

    for (mut cam_transform, _) in camera.iter_mut() {
        let direction = target_transform.translation - cam_transform.translation;
        let distance = direction.length();
        if distance > 1.0 {
            let move_distance = distance * 5.0 * time.delta_secs();
            let movement = direction.normalize() * move_distance.min(distance);
            cam_transform.translation += movement;
        }
    }

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    let x = 0.0 as f32;
    let y = 0.0 as f32;
    let w = 1280.0 as f32;
    let h = 2048.0 as f32;

    min_x = min_x.min(x);
    max_x = max_x.max(x + w);
    min_y = min_y.min(y);
    max_y = max_y.max(y + h);

    for (mut cam_transform, projection) in camera.iter_mut() {
        let Projection::Orthographic(ortho) = projection else {
            continue;
        };

        let half_width = (ortho.area.max.x - ortho.area.min.x) * 0.5;
        let half_height = (ortho.area.max.y - ortho.area.min.y) * 0.5;

        let mut pos = cam_transform.translation;

        let min_cam_x = min_x + half_width;
        let max_cam_x = max_x - half_width;
        let min_cam_y = min_y + half_height;
        let max_cam_y = max_y - half_height;

        pos.x = pos.x.clamp(min_cam_x, max_cam_x);
        pos.y = pos.y.clamp(min_cam_y, max_cam_y);

        cam_transform.translation = pos;
    }}
