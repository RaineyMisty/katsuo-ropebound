// use bevy::prelude::*;
// use super::loader::MapDimensions;
// use crate::app::{FollowedPlayer, MainCamera};

// pub fn camera_follow(
//     map_dim: Res<MapDimensions>,
//     windows: Query<&Window>,
//     mut q_cam: Query<&mut Transform, (With<Camera>, With<MainCamera>, Without<FollowedPlayer>)>,
//     q_player: Query<&Transform, (With<FollowedPlayer>, Without<Camera>)>,
// ) {
//     let Ok(mut cam_t) = q_cam.get_single_mut() else { return; };
//     let Ok(player_t) = q_player.get_single() else { return; };

//     let (half_w, half_h) = if let Ok(win) = windows.get_single() {
//         (win.width() * 0.5, win.height() * 0.5)
//     } else { (640.0, 360.0) };

//     let max_y = (map_dim.h as f32 - half_h).max(half_h);
//     cam_t.translation.y = player_t.translation.y.clamp(half_h, max_y);
// }
