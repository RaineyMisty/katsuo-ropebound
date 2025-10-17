// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Event>
use bevy::prelude::*;

/** Physics event
  *
  * rope -> physics
  */
#[derive(Event, Debug)]
pub struct ForceEvent {
    pub target: Entity,
    pub force: Vec2,
}


/* Impulse event
  *
  * physics -> physics
  * collision -> physics
  */
#[derive(Event, Debug)]
pub struct ImpulseEvent {
    pub target: Entity,
    pub impulse: Vec2,
}

/** Player Intent event
  *
  * control -> physics
  */
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

/* Player Spawn event
 *
 * playerlife -> player
 */
#[derive(Event, Debug)]
pub struct PlayerSpawnEvent {
    // pub name: String,
    pub texture: Handle<Image>,
    pub position: Vec2,
    pub controls: ControlSpec,
    pub mass: Option<f32>,
}

/* Player control spec
 *
 * player -> control
 */
#[derive(Event, Debug)]
pub struct RequestControl {
    pub entity,
    pub spec: ControlSpec
}

#[derive(Debug, Clone)]
pub enum ControlSpec {
    Keyboard {
        up: KeyCode,
        left: KeyCode,
        right: KeyCode,
    },
    Aibot,
}