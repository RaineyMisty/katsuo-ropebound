// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Player components>
use bevy::prelude::*;

#[derive(Component, Clone)]
pub(super) struct Player;

#[derive(Component, Clone)]
pub(super) struct ControlScheme {
    pub up: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}