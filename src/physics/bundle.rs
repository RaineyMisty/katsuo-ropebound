// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <General physics bundle>
use bevy::prelude::*;

use super::component::{Velocity, NetForce, Gravity, Mass, Momentum};

#[derive(Bundle, Clone)]
pub struct PhysicsBundle {
    pub velocity: Velocity,
    pub net_force: NetForce,
    pub gravity: Gravity,
    pub mass: Mass,
    pub momentum: Momentum,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        PhysicsBundle {
            velocity: Velocity::default(),
            net_force: NetForce::default(),
            gravity: Gravity::default(),
            mass: Mass::default(),
            momentum: Momentum::default(),
        }
    }
}

impl PhysicsBundle {
    pub fn new(mass: f32, gravity: bool) -> Self {
        PhysicsBundle {
            velocity: Velocity::default(),
            net_force: NetForce::default(),
            gravity: Gravity(gravity),
            mass: Mass(mass),
            momentum: Momentum::default(),
        }
    }
    pub fn get_mass(&self) -> f32 {
        self.mass.0
    }
    pub fn get_gravity(&self) -> bool {
        self.gravity.0
    }
    pub fn get_velocity(&self) -> Vec2 {
        self.velocity.0
    }
    pub fn get_net_force(&self) -> Vec2 {
        self.net_force.0
    }
    pub fn get_momentum(&self) -> Vec2 {
        self.momentum.0
    }
}