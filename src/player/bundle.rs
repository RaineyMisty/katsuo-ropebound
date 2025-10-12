use bevy::prelude::*;
use crate::player::config::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub control: ControlScheme,
    pub intent: PlayerIntent,
}

impl PlayerBundle {
    pub fn new(controls: ControlScheme, texture: Handle<Image>, transform: Transform) -> Self {
        Self {
            sprite: Sprite {
                image: texture,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            transform,
            control: controls,
            intent: PlayerIntent::default(),
        }
    }
}