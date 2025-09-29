use bevy::prelude::*;
use crate::util::map::data::{EntityData, MapTextureHandles};

use super::data::{MapFile, Boundary, AtlasLayoutResource};

// This represents a platform object.
#[derive(Bundle)]
pub struct MyAtlasSpriteBundle {
    sprite: Sprite,
    transform: Transform,
    visibility: Visibility,
    name: Name,
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

pub fn spawn_map_entity(
    map_dimentions: (u32, u32),
    entity: &EntityData,
    id: &String,
    atlas: &Res<AtlasLayoutResource>,
    images: &Res<MapTextureHandles>,
) -> MyAtlasSpriteBundle {

    let index = atlas.indices[id];

    let position = entity_position(&entity.boundary, 
        (map_dimentions.1) as f32);

    // Spawn entities from entity map.
    // will match here later for entity_type
    let bundle = make_platform(
        images.entity.clone(),
        atlas.layout.clone(),
        index,
        id.clone(),
        (position.x, position.y),
    );

    bundle
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
