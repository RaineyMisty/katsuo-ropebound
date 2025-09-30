// src/util/map/loader.rs
use bevy::prelude::*;
use std::path::Path;

use super::MAP_NAME;
use super::data::{MapFile};
use super::bundles::{MapTextureHandles, AtlasLayoutResource};
use super::util::{full_image, atlas_layout, entity_bundles};

pub fn load_json_map_data(map_name: &str) -> MapFile {
    let json_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(map_name)
        .join(format!("{map_name}.json"));

    let json_str = std::fs::read_to_string(&json_path)
        .expect("Failed to read JSON file");

    serde_json::from_str(&json_str).expect("Failed to parse JSON into MapFile")
}

pub fn load_map_resouces(mut commands: Commands, asset_server: Res<AssetServer>, mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let map: MapFile = load_json_map_data(MAP_NAME);

    let tile_fg_handle = asset_server.load(&map.layer_images.tile_fg);
    let entity_handle = asset_server.load(&map.layer_images.entity);

    let texture_atlas = atlas_layout(&map, &mut atlas_layouts);
    // define entity textures.
    //

    commands.insert_resource(texture_atlas);
    commands.insert_resource(map);

    // I dont see why this is necessary if every game object should be able to point to its asset.
    commands.insert_resource(MapTextureHandles {
        tile_fg: tile_fg_handle,
        entity: entity_handle,
    });
}

pub fn load_map(mut commands: Commands, map: Res<MapFile>, images: Res<MapTextureHandles>, atlas: Res<AtlasLayoutResource>) {
    
    let map_width = map.metadata.cols * map.metadata.tile_size_px;
    let map_height = map.metadata.rows * map.metadata.tile_size_px;

    // load in the tileFG as one full image sprite.
    commands.spawn(full_image(
        &(map_width, map_height),
        &(images.tile_fg),
        -1.0
    ));

    let map_entities = entity_bundles(&images.entity, &atlas, &map);
    for bundle in map_entities {
        commands.spawn(bundle);
    }
}
