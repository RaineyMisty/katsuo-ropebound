use bevy::prelude::*;
use crate::player::config::*;

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(texture: Handle<Image>, transform: Transform) -> Self {
        Self {
            sprite: Sprite {
                image: texture,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            transform,
        }
    }
}