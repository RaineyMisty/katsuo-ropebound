use bevy::prelude::*;

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
