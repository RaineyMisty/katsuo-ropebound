use super::game_object_builder::{Collider, GameObject};
use super::mapdata::{Boundary, EntityData};
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

// Questions Collider relative to game object
// Setting optional properties.
//
// Bevy data structs

// #[derive(Bundle)]
// pub struct BaseGameEntity {
//     pub sprite: Sprite,
//     pub transform: Transform,
//     pub visibility: Visibility,
// }
//
// #[derive(Bundle)]
// pub struct Platform {
//     #[bundle()]
//     pub base: BaseGameEntity,
//     pub name: Name,
//     pub collider: Collider,
// }

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
                (map_height as f32) - (c.start_y + c.height) - parent_boundary.start_y,
            ) + half_extents;

            Collider {
                aabb: Aabb2d::new(local_center, half_extents),
            }
        })
        .unwrap_or(Collider {
            aabb: Aabb2d::new(Vec2::ZERO, Vec2::ZERO),
        })
}

#[derive(Component, Default)]
pub struct Platform;

/// Build a platform entity with sprite + transform + optional collider
/// most of this is probably reused for other entities.
pub fn platform(
    id: &str,
    index: usize,
    entity: &EntityData,
    image: &Handle<Image>,
    atlas_layout: &Handle<TextureAtlasLayout>,
    map_height: u32,
) -> GameObject {
    let collider = collider_from_boundary(entity.collision.as_ref(), &entity.boundary, map_height);

    let sprite = Sprite {
        image: image.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: atlas_layout.clone(),
            index,
        }),
        ..Default::default()
    };

    let transform = Transform::from_xyz(entity.boundary.start_x, entity.boundary.start_y, 0.0);

    let platform = GameObject::new(id, sprite, transform, Visibility::default())
        .with_collider(collider)
        .with_marker::<Platform>();
    platform
}
