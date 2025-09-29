// src/main.rs
mod util;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

use util::map::loader::{load_map_resouces, load_map};
use util::map::MapFile;
use util::dev_mode::move_camera_with_arrows;

use crate::util::map::Collider;
use bevy::color::Color;


pub fn draw_colliders(
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
// Component used for controlling camera in dev_mode
#[derive(Component)]
pub struct CameraController;



// 
fn camera_start(mut commands: Commands, map: Res<MapFile>) {
    let screen = &map.metadata.screen_size;
    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(
                 screen.w as f32 / 2.0,
                 screen.h as f32 / 2.0,
                 0.0,
             ),
             scale: Vec3::splat(1.0),
             ..Default::default() 
        },
        CameraController,
    ));
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Arrow Key Camera Movement Example"),
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin) .add_systems(Startup, (load_map_resouces, camera_start, load_map).chain())
        .add_systems(Update, move_camera_with_arrows)
        .add_systems(Update, draw_colliders)
        .run();
}
