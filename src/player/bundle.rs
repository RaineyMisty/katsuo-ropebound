use bevy::prelude::*;
use crate::player::config::*;
use crate::physics::bundle::PhysicsBundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub player: Player,
    pub physics: PhysicsBundle,
    pub transform: Transform,
    pub control: ControlScheme,
}

impl PlayerBundle {
    pub fn new(controls: ControlScheme, texture: Handle<Image>, transform: Transform) -> Self {
        Self {
            sprite: Sprite {
                image: texture,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            player: Player { controls },
            physics: PhysicsBundle::default(),
            transform,
        }
    }
}