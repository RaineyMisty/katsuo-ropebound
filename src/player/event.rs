// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player event>
use bevy::prelude::*;

use super::component::ControlScheme;

#[derive(Event, Debug)]
pub struct PlayerSpawnEvent {
    // pub name: String,
    pub texture: Handle<Image>,
    pub position: Vec2,
    pub controls: ControlScheme,
    pub mass: Option<f32>,
}