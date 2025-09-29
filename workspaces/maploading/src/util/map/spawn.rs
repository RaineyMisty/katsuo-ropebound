use bevy::prelude::*;
use std::collections::HashMap;
use super::data::{MapFile, Boundary};

#[derive(Bundle)]
pub struct MyAtlasSpriteBundle {
    sprite: Sprite,
    transform: Transform,
    visibility: Visibility,
    name: Name,
}


fn create_atlas_layout(
    map_data: &Res<MapFile>,
    texture_size: UVec2,
) -> (TextureAtlasLayout, HashMap<String, usize>) {
    let mut layout = TextureAtlasLayout::new_empty(texture_size);

    let atlas_indices = map_data
        .entities
        .iter()
        .map(|(entity_id, entity)| {
            let b = &entity.boundary;
            let rect = URect::new(
                b.start_x as u32,
                b.start_y as u32,
                (b.start_x + b.width) as u32,
                (b.start_y + b.height) as u32,
            );
            let index = layout.add_texture(rect);
            (entity_id.clone(), index)
        })
        .collect::<HashMap<_, _>>();

    (layout, atlas_indices)
}

// transform entity position to fit the map layout.
// x - midpoint of rect
// y - midpoint of rect translation flipped.
fn entity_position(b: &Boundary, map_height: f32) -> Vec3 {
    Vec3::new(
        b.start_x + b.width / 2.0,
        map_height - (b.start_y + b.height / 2.0),
        0.0,
    )
}

pub fn spawn_map_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_data: Res<MapFile>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("level1/entity.png");
    let map_size = UVec2::new(
        map_data.metadata.cols * map_data.metadata.tile_size_px,
        map_data.metadata.rows * map_data.metadata.tile_size_px,
    );

    let (layout, atlas_indices) = create_atlas_layout(&map_data, map_size);
    let layout_handle = atlas_layouts.add(layout);

    map_data.entities.iter().for_each(|(id, entity)| {
        let index = atlas_indices[id];

        let position = entity_position(&entity.boundary, map_size.y as f32);

        // Spawn entities from entity map.
        // Could match here later if different kinds spawn different bundles
        let bundle = make_platform(
            texture_handle.clone(),
            layout_handle.clone(),
            index,
            id.clone(),
            (position.x, position.y),
        );

        commands.spawn(bundle);
    });
}

// this function makes no sense, it should return a base object that can be extended
// this functionality remains consistent between all entities.
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
