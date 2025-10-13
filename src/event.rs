// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Event>
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ForceEvent {
    pub target: Entity,
    pub force: Vec2,
}

#[derive(Event, Debug)]
pub struct ImpulseEvent {
    pub target: Entity,
    pub impulse: Vec2,
}

#[derive(Event, Debug)]
pub struct PlayerIntentEvent {
    pub player: Entity,
    pub intent: PlayerIntentKind,
}

#[derive(Debug, Clone)]
pub enum PlayerIntentKind {
    Move{ axis_x: f32 },
    JumpStart,
    JumpHold{ dt: f32 },
    JumpEnd,
}