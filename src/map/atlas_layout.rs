use bevy::prelude::*;
use std::collections::HashMap;
use super::MapFile;

#[derive(Resource)]
pub struct AtlasLayoutResource {
    pub layout: Handle<TextureAtlasLayout>,
    pub indices: HashMap<String, usize>,
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

// build atlas layout from the boundaries defined in the MapFile
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
                // Urect expects left centered origins
                // Translate to left centered origins.
                (b.start_x - b.width / 2.0) as u32,
                (texture_size.y as f32 - (b.start_y + b.height / 2.0)) as u32,
                (b.start_x + b.width / 2.0) as u32,
                (texture_size.y as f32 - (b.start_y - b.height / 2.0)) as u32,
            );
            let index = layout.add_texture(rect);
                (entity_id.clone(), index)
            })
            .collect::<HashMap<_, _>>();

    (layout, atlas_indices)
}

