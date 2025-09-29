use bevy::prelude::*;

#[derive(Component)]
pub struct FullscreenSprite;

use super::data::CameraController;

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

pub fn camera_start(screen: (u32, u32)) -> impl Bundle {
    (
        Camera2d,
        Transform {
            translation: Vec3::new(
                screen.0 as f32 / 2.0,
                screen.1 as f32 / 2.0,
                0.0, // keep positive z so it's above everything
            ),
            scale: Vec3::splat(1.0),
            ..Default::default() },
        CameraController,
    )
}
