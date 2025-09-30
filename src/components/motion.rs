// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player bundle and components>

use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Velocity (pub Vec2);

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct NetForce (pub Vec2);

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Momentum (pub Vec2);

#[derive(Component, Clone, Copy, Debug)]
pub struct Mass (pub f32);

impl Default for Mass {
    fn default() -> Self {
        Mass(1.0)
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Gravity (pub bool);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(true)
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct ControlForce (pub Vec2);

#[derive(Component, Clone, Copy, Debug)]
pub struct RopeForce (pub Vec2);