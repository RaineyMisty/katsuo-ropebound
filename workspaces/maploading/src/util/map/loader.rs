// src/util/map/loader.rs
use bevy::prelude::*;
use std::path::Path;

use super::SCREEN_WIDTH;
use super::SCREEN_HEIGHT;
use super::MAP_NAME;

use super::data::{MapFile, MapTextureHandles};
use super::bundlebuilder::{full_image, camera_start};


pub fn load_map_from_json(map_name: &str) -> MapFile {
    let json_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(map_name)
        .join(format!("{map_name}.json"));

    let json_str = std::fs::read_to_string(&json_path)
        .expect("Failed to read JSON file");

    serde_json::from_str(&json_str).expect("Failed to parse JSON into MapFile")
}

pub fn load_map_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = load_map_from_json(MAP_NAME);
    let tile_fg_handle = asset_server.load(&map.layer_images.tile_fg);
    let entity_handle = asset_server.load(&map.layer_images.entity);

    // load in the tileFG as one full image sprite.
    commands.spawn(full_image(
        &(map.metadata.tile_size_px * map.metadata.cols, map.metadata.tile_size_px * map.metadata.rows),
        &tile_fg_handle,
        -1.0,
    ));

    commands.spawn(camera_start((SCREEN_WIDTH, SCREEN_HEIGHT)));

    commands.insert_resource(map);
    // I dont see why this is necessary if every game object should be able to point to its asset.
    commands.insert_resource(MapTextureHandles {
        tile_fg: tile_fg_handle,
        entity: entity_handle,
    });
}
