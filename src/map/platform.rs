use bevy::prelude::*;
use bevy::math::bounding::Aabb2d;
use super::{mapdata::{Boundary, EntityData}};

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

// impl Collider {
//     pub fn new(width: f32, height: f32, offset: Vec2) -> Self {
//         Self {
//             size: Vec2::new(width, height),
//             offset,
//         }
//     }
//
//     pub fn min_max(&self, pos: Vec2) -> (Vec2, Vec2) {
//         let min = pos + self.offset - self.size / 2.0;
//         let max = pos + self.offset + self.size / 2.0;
//         (min, max)
//     }
//     pub fn halfed(&self) -> Vec2 {
//         self.size * 0.5
//     }
// }

/// Convert a map `Boundary` to a Bevy Aabb2d.
// fn collider_from_boundary(
//     collision: Option<&Boundary>,
// ) -> Collider {
//     collision.map(|c| {
//         let center = Vec2::new(c.start_x, c.start_y);
//         let half_extents = Vec2::new(c.width, c.height) * 0.5;
//         Collider {
//             aabb: Aabb2d::new(center, half_extents),
//         }
//     }).unwrap_or(Collider {
//         aabb: Aabb2d::new(Vec2::ZERO, Vec2::ZERO),
//     })
// }
fn collider_from_boundary(
    collision: Option<&Boundary>,
    parent_boundary: &Boundary,
) -> Collider {
    collision
        .map(|c| {
            let half_extents = Vec2::new(c.width, c.height) * 0.5;

            let local_center = Vec2::new(
                c.start_x - parent_boundary.start_x,
                (64.0*32.0) - (c.start_y+c.height) - parent_boundary.start_y,
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
) -> Platform {
    let collider = collider_from_boundary(entity.collision.as_ref(), &entity.boundary);

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
