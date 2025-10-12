// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Event>
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ForceEvent {
    pub target: Entity,
    pub force: Vec2,
    pub kind: ForceKind,
}

#[derive(Debug, Clone)]
pub enum ForceKind {
    RopeTension { rope: Entity },
    PlayerPush { player: Entity },
}