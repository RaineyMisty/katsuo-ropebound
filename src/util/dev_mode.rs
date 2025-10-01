use bevy::{prelude::*};
use crate::map::Collider;
use crate::app::MainCamera;

// used for deciding at runtime if debug mode is toggled on or off.
#[derive(Resource)]
pub struct Debug(pub bool);

// if false; Debug will not be inserted and cannot be toggled at runtime
const DEBUG: bool = true;

// toggle on and off if debug mode is true.
pub fn toggle_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<Debug>,
) {
    if keyboard_input.just_pressed(KeyCode::Backslash) {
        debug.0 = !debug.0;
    }
}

fn debug_on(debug: Res<Debug>) -> bool {
    debug.0
}

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
    mut query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let speed = 500.0;
        let dt = time.delta_secs();

        if keys.pressed(KeyCode::KeyI) && transform.translation.y < ((64.0*32.0)-(720.0/2.0)) {
            transform.translation.y += speed * dt;
        }
        else if keys.pressed(KeyCode::KeyK) && transform.translation.y >= (720.0/2.0) {
            transform.translation.y -= speed * dt;
        }
    } else {
        // (optional) log once if camera isn't found
        // info!("No camera found with CameraController");
    }
}

fn draw_colliders(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Collider)>,
) {
    for (transform, collider) in &query {
        // Center of the rectangle in 2D
        let position_2d = transform.translation.truncate() + collider.offset;

        // Draw a rectangle centered on the entity's position
        gizmos.rect_2d(
            position_2d,
            collider.size,
            Color::srgba(1.0, 1.0, 1.0, 0.8),
        );
    }
}

pub struct DevModePlugin;

impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        if DEBUG {
            app
                .insert_resource(Debug(false))
                .add_systems(Update, toggle_debug)
                .add_systems(
                    Update,
                    (
                        draw_colliders,
                        move_camera_with_arrows,
                    ).run_if(debug_on)
                );
        }
    }
}
