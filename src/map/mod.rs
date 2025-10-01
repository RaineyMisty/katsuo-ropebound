mod mapdata;
mod platform;
mod atlas_layout;

pub use mapdata::{MapFile, EntityAttrs, Moving};
pub use platform::{Collider};
pub mod loader;

const MAP_NAME: &str = "level1";
pub const SCREEN: (f32, f32) = (1280.0, 720.0);
