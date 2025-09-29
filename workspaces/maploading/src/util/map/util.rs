use bevy::prelude::*;
use super::data::{MapFile, AtlasLayoutResource};
use std::collections::HashMap;
use crate::util::map::{data::{EntityKind::Platform, EntityKind::Coin, Boundary}, entity_builder::{EntityFactory, GameEntityBundle}};

#[derive(Component)]
pub struct FullscreenSprite;

pub fn full_image(
    map_dimentions: &(u32, u32),
    image_handle: &Handle<Image>,
    z_layer: f32,
) -> impl Bundle {
    (
        Sprite::from_image(image_handle.clone()),
        // transform so that map image is loaded as the visual bottom of the screen / where the
        // camera starts.
        Transform::from_xyz(map_dimentions.0 as f32 / 2.0 , map_dimentions.1 as f32 / 2.0, z_layer),
        FullscreenSprite,
    )
}

// game objects -> slice of the entity layer image
pub fn atlas_layout(
    map_data: &MapFile,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> AtlasLayoutResource {
    let texture_size = UVec2::new(
        map_data.metadata.cols * map_data.metadata.tile_size_px,
        map_data.metadata.rows * map_data.metadata.tile_size_px,
    );

    let (layout, indices) = build_layout(map_data, texture_size);
    let layout_handle = atlas_layouts.add(layout);

    AtlasLayoutResource {
        layout: layout_handle,
        indices,
    }
}

fn build_layout(
    map_data: &MapFile,
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

fn entity_position(b: &Boundary, map_height: f32) -> Vec3 {
    Vec3::new(
        b.start_x + b.width / 2.0,
        map_height - (b.start_y + b.height / 2.0),
        0.0,
    )
}
// data that points to the image and the associated layout. for a group of entity objects from the map.
pub fn build_entity_bundles(
    factory: &Res<EntityFactory>,
    atlas: &Res<AtlasLayoutResource>,
    map_data: &Res<MapFile>,
) -> Vec<GameEntityBundle> {
    let map_height = (map_data.metadata.rows * map_data.metadata.tile_size_px) as f32;
    let mut bundles = Vec::new();

    for (id, entity) in &map_data.entities {
        let mut builder = factory.builder()
            .id(id)
            .index(atlas.indices[id])
            .position(entity_position(&entity.boundary, map_height));

        if let Some(collision) = &entity.collision {
            let center_x = collision.start_x + collision.width / 2.0;
            let center_y = map_height - (collision.start_y + collision.height / 2.0);
            let offset_x = center_x - entity_position(&entity.boundary, map_height).x;
            let offset_y = center_y - entity_position(&entity.boundary, map_height).y;

            builder = builder.collider(
                collision.width,
                collision.height,
                Vec2::new(offset_x, offset_y),
            );
        } else {
            warn!("No collision shape defined on entity: {}", id);
        }

        // We always produce the same GameEntityBundle now
        bundles.push(builder.make_bundle());
    }

    bundles
}
