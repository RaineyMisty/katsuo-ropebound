use bevy::prelude::*;

use crate::{Screen, CameraController};

#[derive(Component)]
pub struct FullscreenSprite;

pub fn full_image(
    map_dimentions: &(u32, u32),
    image_handle: &Handle<Image>,
    z_layer: f32,
) -> impl Bundle {
    (
        Sprite::from_image(image_handle.clone()),
        Transform::from_xyz(map_dimentions.0 as f32 / 2.0 , map_dimentions.1 as f32 / 2.0, z_layer),
        FullscreenSprite,
    )
}

pub fn camera_start(screen: Screen) -> impl Bundle {
    (
        Camera2d,
        Transform {
            translation: Vec3::new(
                screen.x as f32 / 2.0,
                screen.y as f32 / 2.0,
                0.0, // keep positive z so it's above everything
            ),
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        CameraController,
    )
}
