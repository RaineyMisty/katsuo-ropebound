mod mapdata;
mod platform;
mod atlas_layout;

pub use mapdata::{MapFile, EntityAttrs, Moving};
pub use platform::{Collider};
pub mod loader;

const MAP_NAME: &str = "level1";
