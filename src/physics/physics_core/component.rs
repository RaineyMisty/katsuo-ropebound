// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Physics components>

use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub(in crate::physics) struct Velocity (pub Vec2);

#[derive(Component, Default, Clone, Copy, Debug)]
pub(in crate::physics) struct NetForce (pub Vec2);

#[derive(Component, Default, Clone, Copy, Debug)]
pub(super) struct Momentum (pub Vec2);

#[derive(Component, Default, Clone, Copy, Debug)]
pub(in crate::physics) struct Impulse (pub Vec2);

#[derive(Component, Clone, Copy, Debug)]
pub(super) struct Mass (pub f32);

impl Default for Mass {
    fn default() -> Self {
        Mass(1.0)
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub(super) struct Gravity (pub bool);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(true)
    }
}

