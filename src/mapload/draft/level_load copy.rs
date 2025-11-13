// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author: Tingxu Chen
// Description: <load level>
use bevy::prelude::*;
use bevy::asset::{AssetLoader, io::Reader, LoadContext};
use bevy::utils::BoxedFuture;
use std::fs;
use anyhow::Result;

use super::resource::{LevelRes, Level};

pub(super) fn load_level_from_ron(mut commands: Commands) {
    let path = "assets/levels/level1.ron";

    let ron_str = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path));

    let level: Level = ron::from_str(&ron_str)
        .unwrap_or_else(|e| panic!("Failed to parse RON {}: {e}", path));

    info!(
        "Loaded level from RON: version={}, objects={}",
        level.version,
        level.objects.len()
    );

    commands.insert_resource(LevelRes(level));
}

// AssetServer

#[derive(Default)]
pub struct LevelRonLoader;

impl AssetLoader for LevelRonLoader {
    type Asset = Level;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _ctx: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let level: Level = ron::de::from_bytes(&bytes)?;
            Ok(level)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}


#[derive(Resource)]
pub struct CurrentLevelHandle(pub Handle<Level>);

#[derive(Resource, Default)]
pub struct DebugLabelsSpawned(pub bool);

pub struct LevelLoadPlugin;

impl Plugin for LevelLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<Level>()
            .init_asset_loader::<LevelRonLoader>()
            .init_resource::<DebugLabelsSpawned>()
            .add_systems(Startup, load_level);
    }
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Level> = asset_server.load("levels/level1.ron");
    commands.insert_resource(CurrentLevelHandle(handle));
}