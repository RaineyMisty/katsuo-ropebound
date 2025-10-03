use bevy::math::bounding::Aabb2d;
// src/util/map/loader.rs
use bevy::prelude::*;
use std::path::Path;

use super::{ MAP_NAME, MapFile };
use super::platform::{platform, Platform, Collider};
use super::atlas_layout::{atlas_layout, AtlasLayoutResource};

#[derive(Resource)]
pub struct MapDimensions { 
    pub w: u32,
    pub h: u32,
}

#[derive(Resource)]
pub struct MapTextureHandles {
    pub tile_fg: Handle<Image>,
    pub entity: Handle<Image>,
}

#[derive(Component)]
pub struct FullscreenSprite;


pub fn background_layer(
    map_dimentions: &(u32, u32),
    image_handle: &Handle<Image>,
    z_layer: f32,
) -> impl Bundle {
    (
        Sprite::from_image(image_handle.clone()),
        // transform so that map image is loaded as the visual bottom of the screen / where the camera starts.
        Transform::from_xyz(map_dimentions.0 as f32 / 2.0 , map_dimentions.1 as f32 / 2.0, z_layer),
        FullscreenSprite,
    )
}

fn load_json_map_data(map_name: &str) -> MapFile {
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

    let map_width = map.metadata.cols * map.metadata.tile_size_px;
    let map_height = map.metadata.rows * map.metadata.tile_size_px;
    commands.insert_resource(MapDimensions {w: map_width, h:map_height});

    commands.insert_resource(texture_atlas);
    commands.insert_resource(map);


    // I dont see why this is necessary if every game object should be able to point to its asset,
    commands.insert_resource(MapTextureHandles {
        tile_fg: tile_fg_handle,
        entity: entity_handle,
    });
}

// data that points to the image and the associated layout. for a group of entity objects from the map.
// this should be made into maybe a builder or factory pattern.
fn entity_bundles(
    image: &Handle<Image>,
    atlas: &Res<AtlasLayoutResource>,
    map_data: &Res<MapFile>,
    map_height: u32,
) -> Vec<Platform> {
    let mut bundles = Vec::new();

    for (id, entity) in &map_data.entities {

        // match for entity.kind, Platform or Coin enum
        let bundle = platform(
            id,
            atlas.indices[id],
            &entity,
            &image,
            &atlas.layout,
            map_height,
        );
        bundles.push(bundle);
    }

    bundles
}

pub fn make_ground() -> Platform {
    let sprite = Sprite {
        color: Color::srgb(0.3, 0.8, 0.3), // ✅ Optional debug color
        custom_size: Some(Vec2::new(1280.0, 5.0)),
        ..Default::default()
    };

    let transform = Transform::from_xyz(1280.0 / 2.0, -1.0, 0.0);
    let visibility = Visibility::default();

    let collider = super::Collider {
        aabb: Aabb2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1280.0, 5.0) * 0.5,
        ),
    };

    // 👇 Builder replaces the bundle struct
    Platform::new("Ground", sprite, transform, visibility)
        .with_collider(collider)
}

pub fn load_map(mut commands: Commands, map: Res<MapFile>, images: Res<MapTextureHandles>, atlas: Res<AtlasLayoutResource>, map_dimensions: Res<MapDimensions>) {
    
    // load in the tileFG as one full image sprite.
    commands.spawn(background_layer(
        &(map_dimensions.w, map_dimensions.h),
        &(images.tile_fg),
        -1.0
    ));

    let map_entities = entity_bundles(&images.entity, &atlas, &map, map_dimensions.h);

    for game_entity in map_entities {
        game_entity.spawn(&mut commands);
    }
    let ground = make_ground();
    ground.spawn(&mut commands);

}
