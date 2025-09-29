// src/main.rs
mod util;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

use util::map::loader::load_map_data;
use util::map::spawn::spawn_map_entities;
use util::dev_mode::move_camera_with_arrows;

// Component used for controlling camera in dev_mode
#[derive(Component)]
pub struct CameraController;

// Screen size resource.
#[derive(Resource)]
pub struct ScreenSize(pub u32, pub u32);

// 
fn camera_start(mut commands: Commands, screen: Res<ScreenSize>) {
    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(
                 screen.0 as f32 / 2.0,
                 screen.1 as f32 / 2.0,
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
        .insert_resource(ScreenSize(1280, 720))
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (load_map_data, camera_start, spawn_map_entities).chain())
        .add_systems(Update, move_camera_with_arrows)
        .run();
}
