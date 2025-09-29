use crate::Camera2d;
use crate::CameraController;
use crate::MapTextureHandles;
use crate::MapFile;
use bevy::prelude::*;
use crate::Path;
// use serde::Deserialize;
// use crate::util::maploading::spawn_full_image;

const MAP_NAME: &str = "level1";

fn load_map_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // works relative to workspace and relative to root
    let json_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(MAP_NAME)
        .join(format!("{MAP_NAME}.json"));

    let json_str = std::fs::read_to_string(&json_path)
        .expect("Failed to read JSON file");
    
    let map: MapFile = serde_json::from_str(&json_str)
        .expect("Failed to parse JSON into MapFile");


    let tile_fg_handle = asset_server.load(&map.layer_images.tile_fg);
    let entity_handle = asset_server.load(&map.layer_images.entity);

    let (rows, cols) = (&map.metadata.rows, &map.metadata.cols);
    let tile_size = &map.metadata.tile_size_px;
    let map_dimentions = (cols*tile_size, rows*tile_size);

    // spawn_full_image(&mut commands, &asset_server, &map_dimentions, "level1/tile_fg.png", 10.0);

    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(
                1280.0 / 2.0,
                720.0/2.0, 
                0.0, // keep positive z so it's above everything
            ),
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        CameraController,
    ));

    commands.insert_resource(MapTextureHandles {
        tile_fg: tile_fg_handle,
        entity: entity_handle,
    });
    commands.insert_resource(map);
}
