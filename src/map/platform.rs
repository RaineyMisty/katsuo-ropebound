use bevy::prelude::*;
use bevy::math::bounding::Aabb2d;
use super::{mapdata::{Boundary, EntityData}};
use super::SCREEN;

// Questions Collider relative to game object
// Setting optional properties.
//
// Bevy data structs
#[derive(Component, Debug)]
pub struct Collider {
    pub aabb: Aabb2d,
}


#[derive(Bundle)]
pub struct BaseGameEntity {
    pub sprite: Sprite,
    pub transform: Transform,
    pub visibility: Visibility,
}

#[derive(Bundle)]
pub struct Platform {
    #[bundle()]
    pub base: BaseGameEntity,
    pub name: Name,
    pub collider: Collider,
}

fn collider_from_boundary(
    collision: Option<&Boundary>,
    parent_boundary: &Boundary,
    map_height: u32,
) -> Collider {
    collision
        .map(|c| {
            let half_extents = Vec2::new(c.width, c.height) * 0.5;

            let local_center = Vec2::new(
                c.start_x - parent_boundary.start_x,
                (map_height as f32) - (c.start_y+c.height) - parent_boundary.start_y,
            ) + half_extents;

            Collider {
                aabb: Aabb2d::new(local_center, half_extents),
            }
        })
        .unwrap_or(Collider {
            aabb: Aabb2d::new(Vec2::ZERO, Vec2::ZERO),
        })
}

/// Build a platform entity with sprite + transform + optional collider
pub fn platform(
    id: &str,
    index: usize,
    entity: &EntityData,
    image: &Handle<Image>,
    atlas_layout: &Handle<TextureAtlasLayout>,
    map_height: u32,
) -> Platform {
    let collider = collider_from_boundary(entity.collision.as_ref(), &entity.boundary, map_height);

    Platform {
        base: BaseGameEntity {
            sprite: Sprite {
                image: image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout.clone(),
                    index,
                }),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                entity.boundary.start_x,
                entity.boundary.start_y,
                0.0,
            ),
            visibility: Visibility::default(),
        },
        name: Name::new(id.to_string()),
        collider,
    }
}
