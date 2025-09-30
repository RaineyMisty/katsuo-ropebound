use bevy::prelude::*;

use super::data::{MapFile};
use std::collections::HashMap;
use super::{data::{Boundary}};
use super::{bundles::{AtlasLayoutResource, GameEntityBundle, BaseComponents}};

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

// game objects -> slice of the entity layer image
// create the AtlasLayoutResource that we defined
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

fn make_entity_bundle(
    id: &str,
    index: usize,
    position: Vec3,
    collider: Option<(f32, f32)>,
    image: &Handle<Image>,
    atlas_layout: &Handle<TextureAtlasLayout>,
) -> impl Bundle {
    // Build base sprite + transform bundle
    let base = BaseComponents {
        sprite: Sprite {
            image: image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout.clone(),
                index,
            }),
            ..Default::default()
        },
        transform: Transform::from_translation(position),
        visibility: Visibility::default(),
        name: Name::new(String::from(id)),
    };

    GameEntityBundle {
        base,
        collider: collider
            .map(|(w, h)| super::Collider::new(w, h, Vec2::ZERO))
            .unwrap_or_else(|| super::Collider::new(0.0, 0.0, Vec2::ZERO)),
    }
}


fn entity_position(b: &Boundary, map_height: f32) -> Vec3 {
    Vec3::new(
        b.start_x + b.width / 2.0,
        map_height - (b.start_y + b.height / 2.0),
        0.0,
    )
}

// data that points to the image and the associated layout. for a group of entity objects from the map.
pub fn entity_bundles(
    image: &Handle<Image>,
    atlas: &Res<AtlasLayoutResource>,
    map_data: &Res<MapFile>,
) -> Vec<impl Bundle> {
    let map_height = (map_data.metadata.rows * map_data.metadata.tile_size_px) as f32;
    let mut bundles = Vec::new();

    for (id, entity) in &map_data.entities {
        let base_position = entity_position(&entity.boundary, map_height);
        // Optional collider calculation
        let collider = entity.collision.as_ref().map(|collision| {
            let center_x = collision.start_x + collision.width / 2.0;
            let center_y = map_height - (collision.start_y + collision.height / 2.0);

            let offset_x = center_x - base_position.x;
            let offset_y = center_y - base_position.y;

            // width, height — your Collider::new handles offset internally
            (collision.width, collision.height, Vec2::new(offset_x, offset_y))
        });

        // If collider exists, pass its size to make_entity_bundle; otherwise None
        let collider_tuple = collider.map(|(w, h, _)| (w, h));

        let bundle = make_entity_bundle(
            id,
            atlas.indices[id],
            base_position,
            collider_tuple,
            &image,
            &atlas.layout,
        );

        if collider.is_none() {
            warn!("No collision shape defined on entity: {}", id);
        }

        bundles.push(bundle);
    }

    bundles
}
