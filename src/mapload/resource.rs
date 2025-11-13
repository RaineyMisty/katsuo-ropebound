// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <Map resources>
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Resource, Default, Debug, Clone)]
pub(super) struct LevelRes(pub Level);

#[derive(Deserialize, Default, TypePath, Asset, Debug, Clone)]
pub(super) struct Level {
    pub(super) version: String,
    pub(super) pixels_per_meter: f32,
    pub(super) background: String,
    pub(super) parallax: Vec<ParallaxLayer>,
    pub(super) chunks: Vec<Chunk>,
    pub(super) objects: Vec<Object>,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct ParallaxLayer {
    pub(super) image: String,
    pub(super) factor: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct Chunk {
    pub(super) id: String,
    pub(super) rect: (i32, i32, i32, i32), // x,y,w,h
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct Object {
    pub(super) id: Option<String>,
    pub(super) kind: Kind,
    pub(super) pos: (f32, f32),
    pub(super) size: (f32, f32),
    pub(super) rot_deg: f32,
    pub(super) collider: ColliderKind,
    pub(super) sprite: Option<String>,
    pub(super) layer: Option<String>,
    pub(super) z: Option<f32>,
    pub(super) props: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) enum Kind {
    Platform,
    Coin,
    JumpPad,
    Spikes,
    Door,
    Switch,
    Enemy,
    SpawnPoint,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) enum ColliderKind {
    Rect,
    Circle,
    Sensor,
    Poly,
}