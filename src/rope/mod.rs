use bevy::prelude::*;

mod rope_force;
// mod rope_visualization;

mod component;
mod config;

use self::rope_force::rope_tension_system;
// use self::rope_visualization::apply_rope_geometry;
// use self::rope_visualization::init_ropes;
// use self::rope_visualization::compute_rope_geometry;

pub struct RopePlugin;

impl Plugin for RopePlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(RopeGeometry::default())
        // .add_systems(Startup, init_ropes)
        app.add_systems(Update, rope_tension_system)
        // .add_systems(Update, compute_rope_geometry)
        // .add_systems(Update, apply_rope_geometry);
    }
}