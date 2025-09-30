// src/util/map/dat.r
// Screen size resource.
use bevy::math::primitives::Rectangle;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ScreenSize {
    pub w: u32,
    pub h: u32,
}


impl Default for ScreenSize {
    fn default() -> Self {
        Self { w: 1280, h: 720 } // or whatever your desired default is
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

    #[serde(default)]
    pub screen_size: ScreenSize,
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
    pub collision_areas: Vec<Rectangle>,
    pub entities: HashMap<String, EntityData>,
}

