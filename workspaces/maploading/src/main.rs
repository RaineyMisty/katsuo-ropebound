use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

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

// ─── Components ────────────────────────────────────────────────
#[derive(Component)]
pub struct MapEntity;

#[derive(Component)]
pub struct Collider {
    pub bounds: Rectangle,
}

#[derive(Component)]
pub struct EntityId(String);

#[derive(Component)]
struct CameraController;

// ─── Resources ────────────────────────────────────────────────
#[derive(Resource)]
pub struct MapTextureHandles {
    pub tile_fg: Handle<Image>,
    pub entity: Handle<Image>,
}

// ─── Setup Functions ──────────────────────────────────────────
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

    commands.insert_resource(MapTextureHandles {
        tile_fg: tile_fg_handle,
        entity: entity_handle,
    });
    commands.insert_resource(map);
}

// ─── Camera Movement System ───────────────────────────────────
fn move_camera_with_arrows(
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
        if keys.pressed(KeyCode::ArrowDown) && transform.translation.y > (720.0/2.0) {
            transform.translation.y -= speed * dt;
        }
    } else {
        // (optional) log once if camera isn't found
        // info!("No camera found with CameraController");
    }
}

// ─── Spawn Tilemap ────────────────────────────────────────────
fn spawn_map_entities(
    mut commands: Commands,
    map_data: Res<MapFile>,
    map_textures: Res<MapTextureHandles>,
    asset_server: Res<AssetServer>,
) {
    let metadata = &map_data.metadata;
    let map_size = TilemapSize { 
        x: metadata.cols, 
        y: metadata.rows
    };

    let map_height = map_size.y as f32 * 64.0;
    let map_width = map_size.x as f32 * 64.0;
    let window_height = 720.0;
    let scale_factor = map_height / window_height;

    // spawn camera
    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(
                0.0,  // start horizontally centered
                720.0 / 2.0,
                0.0,
            ),
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        CameraController,
    ));

    let texture_handle: Handle<Image> = asset_server.load(format!("{MAP_NAME}/tile_fg.png"));
    info!("Map Size: {:?}", map_size);

    // Layer 1
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let w = map_size.x;
    let h = map_size.y;

    for i in 0..(w * h) {
        let y = i / w;
        let x = i % w;
        let tile_pos = TilePos { x, y };

        // Flip Y for texture indexing so the top row starts at index 0
        let tex_index = (h - 1 - y) * w + x;

        let tile_entity = commands.spawn(TileBundle {
            position: tile_pos,
            texture_index: TileTextureIndex(tex_index),
            tilemap_id: TilemapId(tilemap_entity),
            ..Default::default()
        }).id();

        tile_storage.set(&tile_pos, tile_entity);
    }
    
    let tile_size: TilemapTileSize = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(map_textures.tile_fg.clone()),
        tile_size,
        anchor: TilemapAnchor::BottomCenter,
        // transform: Transform::from_translation(Vec3::new(0.0, -360.0, 0.0)),
        ..Default::default()
    });
}

// ─── Input Systems ────────────────────────────────────────────
fn exit_app(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

fn input_just_pressed(key_code: KeyCode) -> impl Fn(Res<ButtonInput<KeyCode>>) -> bool {
    move |inputs: Res<ButtonInput<KeyCode>>| inputs.just_pressed(key_code)
}

fn handle_keyboard_input(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed");
    }
}

fn handle_mouse_input(mouse_buttons: Res<ButtonInput<MouseButton>>) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        info!("Mouse left click");
    }
}

// ─── App Entry ────────────────────────────────────────────────
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
        .add_systems(
            Update,
            (
                handle_keyboard_input,
                handle_mouse_input,
                move_camera_with_arrows, // 👈 added
                exit_app.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .run();
}
