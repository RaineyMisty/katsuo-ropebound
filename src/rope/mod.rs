use bevy::prelude::*;

mod rope_force;
mod rope_visualization;

mod component;
mod config;

pub(in crate::rope) use self::component::*;
pub(in crate::rope) use self::config::*;

use self::rope_force::rope_tension_system;
use self::rope_visualization::apply_rope_geometry;
use self::rope_visualization::init_ropes, compute_rope_geometry;
use crate::player::player_plugin::spawn_player;

pub struct RopePlugin;

impl Plugin for RopePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RopeGeometry::default())
        .add_systems(Startup, init_ropes.after(spawn_player))
        // .add_systems(Startup, init_ropes)
        .add_systems(Update, rope_tension_system)
        .add_systems(Update, compute_rope_geometry)
        .add_systems(Update, apply_rope_geometry);
    }
}