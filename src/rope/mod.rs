use bevy::prelude::*;
use crate::components::rope::{RopeGeometry};

pub mod rope_visualization;

use self::rope_visualization::{apply_rope_geometry, init_ropes, compute_rope_geometry};
use crate::physics::rope_force::{rope_tension_system, rope_force_to_system};
use crate::player::player_plugin::spawn_player;

pub struct RopePlugin;

impl Plugin for RopePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RopeGeometry::default())
        .add_systems(Startup, init_ropes.after(spawn_player))
        // .add_systems(Startup, init_ropes)
        .add_systems(Update, rope_tension_system)
        .add_systems(Update, rope_force_to_system)
        .add_systems(Update, compute_rope_geometry)
        .add_systems(Update, apply_rope_geometry);
    }
}