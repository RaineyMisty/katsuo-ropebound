use bevy::prelude::*;
use crate::config::player::*;
use crate::components::motion::{Velocity, NetForce};

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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub player: Player,
    pub net_force: NetForce,
    pub velocity: Velocity,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(controls: PlayerControls, texture: Handle<Image>, transform: Transform, velocity: Velocity, net_force: NetForce) -> Self {
        Self {
            sprite: Sprite {
                image: texture,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            player: Player { controls },
            net_force: net_force,
            velocity: velocity,
            transform: transform,
        }
    }
}