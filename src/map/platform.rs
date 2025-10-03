use super::game_object_builder::{Collider, GameObject};
use super::mapdata::{Boundary, EntityData};
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;



// unit struct for marking the type that game_object_builder/GameObject creates.
#[derive(Component, Default)]
pub struct Platform;

// builds platform from json map data with builder
pub fn platform(
    id: &str,
    index: usize,
    entity: &EntityData,
    image: &Handle<Image>,
    atlas_layout: &Handle<TextureAtlasLayout>,
    map_height: u32,
) -> GameObject {
    let collider = collider_from_boundary(entity.collision.as_ref(), &entity.boundary, map_height);


    GameObject::new(id, sprite, transform, Visibility::default())
        .with_collider(collider)
        .with_marker::<Platform>()
}
