use bevy::prelude::*;
use crate::config::player::*;
use crate::components::motion::{Velocity, NetForce, ControlForce, Gravity, RopeForce, Mass, Momentum};

#[derive(Component, Clone)]
pub struct Player {
    pub controls: PlayerControls,
}

#[derive(Clone)]
pub struct PlayerControls {
    pub up: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub player: Player,
    pub gravity: Gravity,
    pub control_force: ControlForce,
    pub rope_force: RopeForce,
    pub net_force: NetForce,
    pub mass: Mass,
    pub momentum: Momentum,
    pub velocity: Velocity,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(controls: PlayerControls, texture: Handle<Image>, transform: Transform, velocity: Velocity, mass: Mass) -> Self {
        Self {
            sprite: Sprite {
                image: texture,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            player: Player { controls },
            gravity: Gravity(true),
            control_force: ControlForce(Vec2::ZERO),
            rope_force: RopeForce(Vec2::ZERO),
            net_force: NetForce(Vec2::ZERO),
            mass,
            momentum: Momentum(Vec2::ZERO),
            velocity,
            transform,
        }
    }
}