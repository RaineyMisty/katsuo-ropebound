use bevy::prelude::*;
mod atlas_layout;
mod game_object_builder;
mod loader;
mod mapdata;
mod platform;

pub use game_object_builder::Collider;
pub use mapdata::MapFile;

use loader::{load_map, load_map_resouces};
const MAP_NAME: &str = "level1";
pub const SCREEN: (f32, f32) = (1280.0, 720.0);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_map_resouces, load_map).chain());
    }
}
