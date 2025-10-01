use bevy::prelude::*;
use super::{mapdata::{Boundary, EntityData}};

// Bevy data structs
#[derive(Component, Debug)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Bundle)]
pub struct GameEntityBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub visibility: Visibility,
    pub name: Name,
    pub collider: Collider,
}

impl Collider {
    pub fn new(width: f32, height: f32, offset: Vec2) -> Self {
        Self {
            size: Vec2::new(width, height),
            offset,
        }
    }

    pub fn min_max(&self, pos: Vec2) -> (Vec2, Vec2) {
        let min = pos + self.offset - self.size / 2.0;
        let max = pos + self.offset + self.size / 2.0;
        (min, max)
    }
    pub fn halfed(&self) -> Vec2 {
        self.size * 0.5
    }
}

fn collider_offset(
    collision: Option<&Boundary>,
    boundary: &Boundary,
) -> Option<(f32, f32, Vec2)> {
    collision.map(|c| {
        let center_x = c.start_x;
        let center_y = c.start_y;

        let offset_x = center_x - boundary.start_x;
        let offset_y = center_y - boundary.start_y;

        (c.width, c.height, Vec2::new(offset_x, offset_y))
    })
}

pub fn platform(
    id: &str,
    index: usize,
    entity: &EntityData,
    image: &Handle<Image>,
    atlas_layout: &Handle<TextureAtlasLayout>,
) -> impl Bundle {
    let collider = collider_offset(entity.collision.as_ref(), &entity.boundary);
    // Build base sprite + transform bundle
    GameEntityBundle {
        sprite: Sprite {
            image: image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout.clone(),
                index,
            }),
            ..Default::default()
        },
        transform: Transform::from_xyz(entity.boundary.start_x, entity.boundary.start_y, 0.0),
        visibility: Visibility::default(),
        name: Name::new(String::from(id)),
        collider: collider
            .map(|(w, h, _)| Collider::new(w, h, Vec2::ZERO))
            .unwrap_or_else(|| Collider::new(0.0, 0.0, Vec2::ZERO)),
    }
}
