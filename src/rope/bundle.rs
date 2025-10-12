// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope bundle>
use bevy::prelude::*;

use super::component::{Rope, SpringJoint, RopeEnds};

#[derive(Bundle, Clone)]
pub struct RopeBundle {
    pub rope: Rope,
    pub spring_joint: SpringJoint,
    pub rope_ends: RopeEnds,
}