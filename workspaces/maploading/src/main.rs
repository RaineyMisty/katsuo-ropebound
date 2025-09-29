mod util;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::util::maploading::*;
use crate::util::dev_mode::*;
use bevy::math::primitives::Rectangle;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;


struct Screen {
    x: u32,
    y: u32,
}
const MAP_NAME: &str = "level1";
const SCREEN: Screen = Screen{x: 1280, y:720};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub tile_size_px: u32,
    pub rows: u32,
    pub cols: u32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Boundary {
    start_x: f32,
    start_y: f32,
    width: f32,
    height: f32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntityData {
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
struct CameraController;

// ─── Resources ────────────────────────────────────────────────
#[derive(Resource)]
pub struct MapTextureHandles {
    pub tile_fg: Handle<Image>,
    pub entity: Handle<Image>,
}

#[derive(Bundle)]
struct MyAtlasSpriteBundle {
    sprite: Sprite,
    transform: Transform,
    visibility: Visibility,
    name: Name,
}

fn load_map_from_json(map_name: &str) -> MapFile {
    let json_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(map_name)
        .join(format!("{map_name}.json"));

    let json_str = std::fs::read_to_string(&json_path)
        .expect("Failed to read JSON file");

    serde_json::from_str(&json_str)
        .expect("Failed to parse JSON into MapFile")
}
// ─── Setup Functions ──────────────────────────────────────────
fn load_map_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let map = load_map_from_json(MAP_NAME);
    // works relative to workspace and relative to root
    let tile_fg_handle = asset_server.load(&map.layer_images.tile_fg);
    let entity_handle = asset_server.load(&map.layer_images.entity);


    commands.spawn(full_image(
        &(map.metadata.tile_size_px * map.metadata.cols, map.metadata.tile_size_px * map.metadata.rows),
        &tile_fg_handle,
        10.0,
    ));

    commands.spawn(camera_start(SCREEN));

    commands.insert_resource(map);
    commands.insert_resource(MapTextureHandles {
        tile_fg: tile_fg_handle,
        entity: entity_handle,
    });

}

// ─── Camera Movement System ───────────────────────────────────
pub fn spawn_entity_rectangles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_data: Res<MapFile>,
) {
    let map_height = map_data.metadata.rows as f32 * map_data.metadata.tile_size_px as f32;

    for (id, entity) in &map_data.entities {
        let b = &entity.boundary;

        let x = b.start_x + b.width / 2.0;
        let y = map_height - (b.start_y + b.height / 2.0);

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


fn create_atlas_layout(map_data: &Res<MapFile>, texture_size: UVec2) -> (TextureAtlasLayout, HashMap<String, usize>) {
    // Create a mutable layout we will populate with subregions for each entity
    let mut layout = TextureAtlasLayout::new_empty(texture_size);

    // Keep track of which atlas index belongs to which entity
    let mut atlas_indices: HashMap<String, usize> = HashMap::new();

    for (_i, (entity_id, entity_data)) in map_data.entities.iter().enumerate() {
        // For now, let's assume each entity gets a 128×128 region laid out horizontally
        let region_x = entity_data.boundary.start_x as u32;
        let region_y = entity_data.boundary.start_y as u32;
        let region_width = entity_data.boundary.width as u32;
        let region_height = entity_data.boundary.height as u32;

        let rect = URect::new(region_x, region_y, region_x + region_width, region_y + region_height);
        let index = layout.add_texture(rect);
        atlas_indices.insert(entity_id.clone(), index);
    }

    (layout, atlas_indices)
}

// try to get a specifc portiton of the image and load it to a specific area.
fn spawn_map_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_data: Res<MapFile>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("level1/entity.png");
    let map_height = (map_data.metadata.rows * map_data.metadata.tile_size_px) as f32;
    let map_width = (map_data.metadata.cols * map_data.metadata.tile_size_px) as f32;
    let texture_size = UVec2::new(map_width as u32, map_height as u32);

    let (layout, atlas_indices) = create_atlas_layout(&map_data, texture_size);

    // Add the layout once after populating it
    let layout_handle = atlas_layouts.add(layout);
    // Spawn one sprite per entity, using its unique atlas index
    for (entity_id, entity) in &map_data.entities {
        let index = atlas_indices[entity_id];

        let b = &entity.boundary;

        let x = b.start_x + b.width / 2.0;
        let y = map_height - (b.start_y + b.height / 2.0);

        let offset = (x,y);

        commands.spawn(match entity.kind.as_str() {
            "platform" => make_platform(texture_handle.clone(), layout_handle.clone(), index, entity_id.clone(), offset),
            _ => make_platform(texture_handle.clone(), layout_handle.clone(), index, entity_id.clone(), offset),
        });
    }
}

// this function makes no sence, it should return a base object that can be extended.
fn make_platform(texture_handle: Handle<Image>, layout_handle: Handle<TextureAtlasLayout>, index: usize, entity_id: String, offset: (f32, f32)) -> MyAtlasSpriteBundle {

    MyAtlasSpriteBundle {
        sprite: Sprite {
            image: texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index,
            }),
            ..default()
        },
        transform: Transform::from_xyz(offset.0, offset.1, 0.0),
        visibility: Visibility::default(),
        name: Name::new(entity_id.clone()),
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
                move_camera_with_arrows, // 👈 added
            ),
        )
        .run();
}
