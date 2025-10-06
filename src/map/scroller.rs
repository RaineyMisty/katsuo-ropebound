use bevy::prelude::*;
use super::loader::MapDimensions; 

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 400.0;

pub fn player_move(
    kb: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q_player: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut t) = q_player.get_single_mut() else { return; };
    let mut dir = Vec2::ZERO;
    if kb.pressed(KeyCode::ArrowLeft)  || kb.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if kb.pressed(KeyCode::ArrowRight) || kb.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    if kb.pressed(KeyCode::ArrowUp)    || kb.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if kb.pressed(KeyCode::ArrowDown)  || kb.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if dir != Vec2::ZERO {
        t.translation += (dir.normalize() * PLAYER_SPEED * time.delta_secs()).extend(0.0);
    }
}

pub fn camera_follow(
    map_dim: Res<MapDimensions>,
    windows: Query<&Window>,

    mut q_cam: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player: Query<&Transform, With<Player>>,
) {
    let Ok(mut cam_t) = q_cam.get_single_mut() else { return; };
    let Ok(pt) = q_player.get_single() else { return; };

    let (half_w, half_h) = if let Ok(win) = windows.get_single() {
        (win.width() * 0.5, win.height() * 0.5)
    } else {
        (640.0, 360.0)
    };

    let max_x = (map_dim.w as f32 - half_w).max(half_w);
    let max_y = (map_dim.h as f32 - half_h).max(half_h);

    cam_t.translation.x = pt.translation.x.clamp(half_w, max_x);
    cam_t.translation.y = pt.translation.y.clamp(half_h, max_y);
}
