use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Bundle)]
pub struct MyAtlasSpriteBundle {
    sprite: Sprite,
    transform: Transform,
    visibility: Visibility,
    name: Name,
}

use super::data::{MapFile};

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

pub fn spawn_map_entities(
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

// this function makes no sense, it should return a base object that can be extended.
pub fn make_platform(texture_handle: Handle<Image>, layout_handle: Handle<TextureAtlasLayout>, index: usize, entity_id: String, offset: (f32, f32)) -> MyAtlasSpriteBundle {

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
