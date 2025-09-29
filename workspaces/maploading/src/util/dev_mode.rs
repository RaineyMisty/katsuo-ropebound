use bevy::prelude::*;
use crate::CameraController;

pub fn log_mouse_position(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let Ok(window) = windows.get_single() else { return };

    if let Some(cursor) = window.cursor_position() {
        // Cursor is measured from bottom-left corner of the window by default
        let x = cursor.x;
        let y = cursor.y;

        if buttons.just_pressed(MouseButton::Left) {
            info!("🖱 Mouse clicked at SCREEN position: x: {:.2}, y: {:.2}", x, y);
        } else {
            info!("Mouse at SCREEN position: x: {:.2}, y: {:.2}", x, y);
        }
    }
}

pub fn move_camera_with_arrows(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<CameraController>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let speed = 500.0;
        let dt = time.delta_secs();

        if keys.pressed(KeyCode::ArrowUp) && transform.translation.y < ((64.0*32.0)-(720.0/2.0)) {
            transform.translation.y += speed * dt;
        }
        else if keys.pressed(KeyCode::ArrowDown) && transform.translation.y >= (720.0/2.0) {
            transform.translation.y -= speed * dt;
        }
    } else {
        // (optional) log once if camera isn't found
        // info!("No camera found with CameraController");
    }
}
