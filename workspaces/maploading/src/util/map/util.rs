use bevy::prelude::*;
use super::data::{MapFile};
use super::data::{MapFile, MapTextureHandles, AtlasLayoutResource};
use std::collections::HashMap;

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
