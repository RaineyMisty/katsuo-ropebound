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
    #[bundle]
    pub sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(controls: PlayerControls, texture: Handle<Image>) -> Self {
        Self {
            player: Player { controls },
            velocity: Velocity::default(),
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_scale(Vec3::splat(0.5)),
                ..Default::default()
            },
        }
    }
}