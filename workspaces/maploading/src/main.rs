use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy::math::primitives::Rectangle;
use bevy::asset::LoadState;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use bevy::render::mesh::{Indices, MeshVertexAttribute};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;

const MAP_NAME: &str = "level1";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub tile_size_px: u32,
    pub rows: u32,
    pub cols: u32,
}

// #[derive(Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Rectangle {
//     pub start_x: f32,
//     pub start_y: f32,
//     pub width: f32,
//     pub height: f32,
// }

#[serde(rename_all = "camelCase")]
#[derive(serde::Deserialize, Debug)]
struct EntityData {
    boundary: Boundary,
    #[serde(rename = "type")]
    kind: String,
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
        else if keys.pressed(KeyCode::ArrowDown) && transform.translation.y >= (720.0/2.0) {
            transform.translation.y -= speed * dt;
        }
        // else if keys.pressed(KeyCode::ArrowLeft) {
        //     transform.translation.x -= speed * dt;
        // }
        // else if keys.pressed(KeyCode::ArrowRight) {
        //     transform.translation.x += speed * dt;
        // }
    } else {
        // (optional) log once if camera isn't found
        // info!("No camera found with CameraController");
    }
}

#[derive(serde::Deserialize, Debug)]
struct Boundary {
    startX: f32,
    startY: f32,
    width: f32,
    height: f32,
}

#[derive(serde::Deserialize, Debug)]
struct EntityFile {
    entities: HashMap<String, EntityData>,
}

fn make_textured_rect(
    width: f32,
    height: f32,
    tex_x: f32,
    tex_y: f32,
    tex_w: f32,
    tex_h: f32,
    atlas_w: f32,
    atlas_h: f32,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

    // Rectangle vertices centered at origin
    let hw = width / 2.0;
    let hh = height / 2.0;

    // Positions (X, Y, Z)
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-hw, -hh, 0.0], // bottom left
            [ hw, -hh, 0.0], // bottom right
            [ hw,  hh, 0.0], // top right
            [-hw,  hh, 0.0], // top left
        ],
    );

    // UV coordinates (normalized)
    let u_min = tex_x / atlas_w;
    let v_min = 1.0 - (tex_y + tex_h) / atlas_h;
    let u_max = (tex_x + tex_w) / atlas_w;
    let v_max = 1.0 - tex_y / atlas_h;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [u_min, v_min],
            [u_max, v_min],
            [u_max, v_max],
            [u_min, v_max],
        ],
    );

    // Indices (two triangles)
    mesh.insert_indices(Indices::U32(vec![
        0, 1, 2,
        0, 2, 3,
    ]));

    mesh
}

pub fn spawn_entity_rectangles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_data: Res<MapFile>,
    map_textures: Res<MapTextureHandles>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let map_height = map_data.metadata.rows as f32 * map_data.metadata.tile_size_px as f32;

    // ✅ Use the handle stored in MapTextureHandles
    match asset_server.get_load_state(&map_textures.entity) {
        Some(LoadState::Loaded) => {
            // ready to use the image
        }
        _ => return,
    }
    let atlas = images.get(&map_textures.entity).unwrap();
    let atlas_size = atlas.size_f32();

    for (id, entity) in &map_data.entities {
        let b = &entity.boundary;

        let x = b.startX + b.width / 2.0;
        let y = map_height - (b.startY + b.height / 2.0);

        let mesh = make_textured_rect(
            b.width,
            b.height,
            b.startX,
            b.startY,
            b.width,
            b.height,
            atlas_size.x,
            atlas_size.y,
        );

        commands.spawn((
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(x, y, 0.0),
            Name::new(format!("Entity {id}")),
        ));
    }
}

// ─── Spawn Tilemap ────────────────────────────────────────────
// make map entities,  
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
                map_width / 2.0,
                720.0 / 2.0, 
                0.0, // keep positive z so it's above everything
            ),
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        CameraController,
    ));
    info!("Map Size: {:?}", map_size);

    // Layer 1
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let w = map_size.x;
    let h = map_size.y;

    // send tileset in json
    // send 2d array of set tiles. 
    // send rotation and flip data
    // iterate map of set tiles.
    //
    // calculate groups.
    // make platform entities out of tile groups.
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
        texture: TilemapTexture::Single(map_textures.entity.clone()),
        tile_size,
        anchor: TilemapAnchor::BottomLeft,
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
        .add_systems(Startup, (load_map_data, spawn_map_entities, spawn_entity_rectangles).chain())
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
