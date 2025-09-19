use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Player {
    pub controls: PlayerControls,
}

#[derive(Clone)]
pub struct PlayerControls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub velocity: Velocity,
}

impl PlayerBundle {
    pub fn new(controls: PlayerControls, texture: Handle<Image>) -> Self {
        Self {
            player: Player { controls },
            velocity: Velocity::default(),
        }
    }
}