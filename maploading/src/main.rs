use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

const MAP_NAME: &str = "level1";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub tile_size_px: u32,
    pub rows: u32,
    pub cols: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rectangle {
    pub start_x: f32,
    pub start_y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntityData {
    pub boundary: Rectangle,
    pub collision: Rectangle,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LayerImages {
    pub tile_fg: String,
    pub entity: String,
}

#[derive(Deserialize, Debug, Resource)]
#[serde(rename_all = "camelCase")]
pub struct MapFile {
    pub metadata: Metadata,
    pub layer_images: LayerImages,
    pub collision_areas: Vec<Rectangle>,
    pub entities: HashMap<String, EntityData>,
}

// 组件定义
#[derive(Component)]
pub struct MapEntity;

#[derive(Component)]
pub struct Collider {
    pub bounds: Rectangle,
}

#[derive(Component)]
pub struct EntityId(String);

// 资源定义
#[derive(Resource)]
pub struct MapTextureHandles {
    pub tile_fg: Handle<Image>,
    pub entity: Handle<Image>,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn load_map_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // works relative to workspace and relative to root
    let json_path = format!(
        "{}/assets/{}/{}.json",
        env!("CARGO_MANIFEST_DIR"),
        MAP_NAME,
        MAP_NAME
    );

    // read Json document
    let json_str = std::fs::read_to_string(&json_path)
        .expect("Failed to read JSON file");
    
    let map: MapFile = serde_json::from_str(&json_str)
        .expect("Failed to parse JSON into MapFile");

    // tile_fg and entitiy point to image path's relative to the asset folder
    let tile_fg_path = map.layer_images.tile_fg.clone();
    let entity_path = map.layer_images.entity.clone();

    // load images as textures
    let tile_fg_handle = asset_server.load(&tile_fg_path);
    let entity_handle = asset_server.load(&entity_path);

    // store texture handles as resource
    commands.insert_resource(MapTextureHandles {
        tile_fg: tile_fg_handle,
        entity: entity_handle,
    });

    // store map data as resource
    commands.insert_resource(map);
}


// define exit system - inspired by logic in image
fn exit_app(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

// check if a specific key was just pressed
fn input_just_pressed(key_code: KeyCode) -> impl Fn(Res<ButtonInput<KeyCode>>) -> bool {
    move |inputs: Res<ButtonInput<KeyCode>>| inputs.just_pressed(key_code)
}

// keep the app running system
fn keep_app_alive() {
    // empty system to keep app alive
    // add game logic here in real app
}

// input handling systems
fn handle_keyboard_input(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        // add logging
        info!("ESC键被按下");
    }
}

// mouse input handling system
fn handle_mouse_input(mouse_buttons: Res<ButtonInput<MouseButton>>) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        info!("鼠标左键点击");
    }
}


fn spawn_map_entities(
    mut commands: Commands,
    map_data: Res<MapFile>,
    map_textures: Res<MapTextureHandles>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // let texture_handle: Handle<Image> = asset_server.load("tileLayer.png");
    let texture_handle: Handle<Image> = asset_server.load(format!("{MAP_NAME}/tile_fg.png"));
    let metadata = &map_data.metadata;
    let map_size = TilemapSize { 
        x: metadata.cols, 
        y: metadata.rows 
    };
    print!("Map Size: {:?}", map_size);

    // Layer 1
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();


    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            
            // core: calculate different texture index for each tile
            let texture_index = x*16+y; 
            
            let tile_entity = commands.spawn(TileBundle {
                position: tile_pos,
                texture_index: TileTextureIndex(texture_index),
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            }).id();
            
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    
    // load entity texture
    let tile_size: TilemapTileSize = TilemapTileSize { x: 80.0, y: 80.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(map_textures.tile_fg.clone()),
        // texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let texture_handle: Handle<Image> = asset_server.load(format!("{MAP_NAME}/tiles.png"));

    let map_size: TilemapSize = TilemapSize { x: 10, y: 10 };

    // Layer 1
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();


    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            
            // core: calculate different texture index for each tile
            let texture_index = x*10+y; 
            
            let tile_entity = commands.spawn(TileBundle {
                position: tile_pos,
                texture_index: TileTextureIndex(texture_index),
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            }).id();
            
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });

    // Layer 2
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    fill_tilemap(
        TileTextureIndex(2),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
        anchor: TilemapAnchor::Center,
        transform: Transform::from_xyz(32.0, 64.0, 1.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Layers Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (load_map_data, spawn_map_entities).chain())
        .add_systems(Update, (
            handle_keyboard_input,    // keyboard input handling
            handle_mouse_input,       // mouse input handling
            exit_app.run_if(input_just_pressed(KeyCode::Escape)), // exit on ESC
            keep_app_alive // keep app alive
        ))
        .run();
}
