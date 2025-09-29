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

    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(
                1280.0 / 2.0,
                720.0 / 2.0, 
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
    world_width: f32,
    world_height: f32,
    tex_x: f32,
    tex_y: f32,
    tex_w: f32,
    tex_h: f32,
    atlas_w: f32,
    atlas_h: f32,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

    // Rectangle vertices centered at origin
    let hw = world_width / 2.0;
    let hh = world_height / 2.0;

    // Geometry in local space
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-hw, -hh, 0.0], // bottom left
            [ hw, -hh, 0.0], // bottom right
            [ hw,  hh, 0.0], // top right
            [-hw,  hh, 0.0], // top left
        ],
    );

    // Convert from top-left pixel coords → bottom-left UV coords
    let u_min = tex_x / atlas_w;
    let u_max = (tex_x + tex_w) / atlas_w;
    let v_max = 1.0 - tex_y / atlas_h;
    let v_min = 1.0 - (tex_y + tex_h) / atlas_h;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [u_min, v_min],
            [u_max, v_min],
            [u_max, v_max],
            [u_min, v_max],
        ],
    );

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
) {
    let map_height = map_data.metadata.rows as f32 * map_data.metadata.tile_size_px as f32;

    for (id, entity) in &map_data.entities {
        let b = &entity.boundary;

        let x = b.startX + b.width / 2.0;
        let y = map_height - (b.startY + b.height / 2.0);

        let color = match entity.kind.as_str() {
            "platform" => Color::srgb(0.2, 0.8, 0.3),
            "enemy" => Color::srgb(0.8, 0.2, 0.2),
            _ => Color::srgba(0.6, 0.6, 0.6, 0.8),
        };

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, y, 0.0)
                .with_scale(Vec3::new(b.width, b.height, 1.0)),
            Name::new(format!("Entity {id}")),
        ));
    }
}

#[derive(Bundle)]
struct MyAtlasSpriteBundle {
    sprite: Sprite,
    transform: Transform,
    visibility: Visibility,
    name: Name,
}

// try to get a specifc portiton of the image and load it to a specific area.
fn spawn_map_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_data: Res<MapFile>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("level1/entity.png");
    let texture_size = UVec2::new(1280, 64 * 32);
    let map_height = map_data.metadata.rows as f32 * map_data.metadata.tile_size_px as f32;

    // Create a mutable layout we will populate with subregions for each entity
    let mut layout = TextureAtlasLayout::new_empty(texture_size);

    // Keep track of which atlas index belongs to which entity
    let mut atlas_indices: HashMap<String, usize> = HashMap::new();

    for (i, (entity_id, entity_data)) in map_data.entities.iter().enumerate() {
        // For now, let's assume each entity gets a 128×128 region laid out horizontally
        let region_x = entity_data.boundary.startX as u32;
        let region_y = entity_data.boundary.startY as u32;
        let region_width = entity_data.boundary.width as u32;
        let region_height = entity_data.boundary.height as u32;

        let rect = URect::new(region_x, region_y, region_x + region_width, region_y + region_height);
        let index = layout.add_texture(rect);
        atlas_indices.insert(entity_id.clone(), index);
    }

    // Add the layout once after populating it
    let layout_handle = atlas_layouts.add(layout);
    // Spawn one sprite per entity, using its unique atlas index
    for (entity_id, entity) in &map_data.entities {
        let index = atlas_indices[entity_id];

        let b = &entity.boundary;

        let x = b.startX + b.width / 2.0;
        let y = map_height - (b.startY + b.height / 2.0);

        commands.spawn(MyAtlasSpriteBundle {
            sprite: Sprite {
                image: texture_handle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: layout_handle.clone(),
                    index,
                }),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            visibility: Visibility::default(),
            name: Name::new(entity_id.clone()),
        });
    }
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
