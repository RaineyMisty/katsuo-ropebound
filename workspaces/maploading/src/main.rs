// src/main.rs
mod util;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

use util::map::loader::load_map_data;
use util::map::spawn::spawn_map_entities;
use util::dev_mode::move_camera_with_arrows; // example

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
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (load_map_data, spawn_map_entities).chain())
        .add_systems(Update, move_camera_with_arrows)
        .run();
}
