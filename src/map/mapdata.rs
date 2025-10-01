// src/util/map/dat.r
// Screen size resource.
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use serde::de::{Deserializer};

pub fn from_int_to_f32<'de, D>(d: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let num = serde_json::Value::deserialize(d)?;
    match num {
        serde_json::Value::Number(n) => {
            n.as_f64()
                .map(|v| v as f32)
                .ok_or_else(|| serde::de::Error::custom("Expected number for f32"))
        }
        _ => Err(serde::de::Error::custom("Expected number for f32")),
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EntityKind {
    Platform,
    Coin,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub tile_size_px: u32,
    pub rows: u32,
    pub cols: u32,

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Boundary {
    pub start_x: f32,
    pub start_y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntityData {
    pub boundary: Boundary,
    #[serde(rename = "type")]
    pub kind: EntityKind,
    pub collision: Option<Boundary>,
    pub attributes: EntityAttrs,
}

#[derive(Component, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EntityAttrs {
    oneWay: bool,
    pub moving: Option<Moving>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MoveType {
    Linear,
    Radial,
    Custom,
}

#[derive(Component, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]  // 👈 important
pub struct Moving {
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
    pub move_type: MoveType,
    #[serde(deserialize_with = "from_int_to_f32")]
    pub speed: f32,
    pub trigger: MovementTrigger,

    // --- New fields for runtime state ---
    pub t: f32,

    pub forward: bool,
}

impl Default for Moving {
    fn default() -> Self {
        Self {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            move_type: MoveType::Linear,
            speed: 1.0,
            trigger: MovementTrigger::default(),
            t: 0.0,
            forward: true,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MovementTrigger {
    pub trigger_type: MovementTriggerType,
}

impl Default for MovementTrigger {
    fn default() -> Self {
        Self {
            trigger_type: MovementTriggerType::default(),
        }
    }
}

#[derive(Component, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MovementTriggerType {
    Loop,
    OnPlayerContact,
}

impl Default for MovementTriggerType {
    fn default() -> Self {
        MovementTriggerType::Loop
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LayerImages {
    pub tile_fg: String,
    pub entity: String,
}

#[derive(Deserialize, Debug, Resource)]
#[serde(rename_all = "camelCase")]
pub struct MapFile {
    pub metadata: Metadata,
    pub layer_images: LayerImages,
    pub collision_areas: Vec<Boundary>,
    pub entities: HashMap<String, EntityData>,
}

