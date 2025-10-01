use bevy::prelude::*;
mod mapdata;
mod platform;
mod atlas_layout;

pub use mapdata::{MapFile};
pub use platform::{Collider};
pub mod loader;

use loader::{load_map, load_map_resouces};
const MAP_NAME: &str = "level1";
pub const SCREEN: (f32, f32) = (1280.0, 720.0);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                load_map_resouces,
                load_map
            ).chain()
        );
    }
}
