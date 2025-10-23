// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope physics>
use bevy::prelude::*;

use crate::event::RegisterRope;

pub(super) fn rope_register_system(
    mut commands: Commands,
    mut events: EventReader<RegisterRope>,
) {
}