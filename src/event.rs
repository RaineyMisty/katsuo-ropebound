// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Event>
use bevy::prelude::*;

// Event plugins
pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RegisterRope>()
           // .add_event::<UnregisterRope>()
           .add_event::<PlayerIntentEvent>()
           .add_event::<PlayerSpawnEvent>()
           .add_event::<PlayerSpawned>()
           .add_event::<RopeSpawnEvent>()
           .add_event::<RequestControl>()
           .add_event::<RequestPlayerPhysics>()
           .add_event::<RequestRopePhysics>();
    }
}

/** Rope registration event
  *
  * rope -> physics
  */
#[derive(Event, Clone, Copy, Debug)]
pub struct RegisterRope {
    pub rope_entity: Entity,
    pub head_entity: Entity,
    pub tail_entity: Entity,
    pub rest_length: f32,
    pub max_extension: f32,
    pub spring_constant: f32,
}

// #[derive(Event, Clone, Copy, Debug)]
// pub struct UnregisterRope {
//     pub rope: Entity,
// }

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
 * player_lifetime -> player
 */
#[derive(Event, Debug)]
pub struct PlayerSpawnEvent {
    // pub name: String,
    pub node: u32,
    pub texture: Handle<Image>,
    pub position: Vec2,
    pub controls: ControlSpec,
    pub mass: Option<f32>,
}

#[derive(Event, Debug)]
pub struct PlayerSpawned {
    pub entity: Entity,
    pub node: u32,
}

/** Rope Spawn event
  *
  * player_lifetime -> rope
  */
#[derive(Event, Debug)]
pub struct RopeSpawnEvent {
    pub head_entity: Entity,
    pub tail_entity: Entity,
}

/* Player control spec
 *
 * player -> control
 */
#[derive(Event, Debug)]
pub struct RequestControl {
    pub entity: Entity,
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

#[derive(Event, Debug)]
pub struct RequestPlayerPhysics {
    pub entity: Entity,
    pub mass: f32,
}

#[derive(Event, Debug)]
pub struct RequestRopePhysics {
    pub entity: Entity,
}