use bevy::prelude::*;
use crate::config::player::*;
use crate::components::motion::{Velocity, NetForce, ControlForce};

#[derive(Component, Clone)]
pub struct Player {
    pub controls: PlayerControls,
    pub control_force: ControlForce,
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
    pub fn new(controls: PlayerControls, control_force: ControlForce, texture: Handle<Image>, transform: Transform, velocity: Velocity, net_force: NetForce) -> Self {
        Self {
            sprite: Sprite {
                image: texture,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            player: Player { controls: controls, control_force: control_force },
            net_force: net_force,
            velocity: velocity,
            transform: transform,
        }
    }
}