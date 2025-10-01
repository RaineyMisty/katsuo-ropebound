// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Create App and setup camera>

use bevy::prelude::*;
use bevy::time::Fixed;
use crate::player::PlayerPlugin;
use crate::physics::PhysicsPlugin;
use crate::config::*;

use crate::map::loader::{load_map, load_map_resouces};
use crate::map::{EntityAttrs, MapFile, Moving, SCREEN};
use crate::util::{DevModePlugin};

use bevy::render::view::RenderLayers;


// Example query for getting the platform colliders which are visible on screen
// fn log_offscreen_entities(
//     q: Query<(Entity, &ViewVisibility), (With<Collider>, With<Transform>)>,
// ) {
//     for (e, view) in &q {
//         if !view.get() {
//             info!("🛰 Entity {:?} with Collider is off-screen", e);
//         }
//     }
// }


// Update the moving.t value with ping-pong behavior
fn update_t(moving: &mut Moving, dt: f32) {
    if moving.forward {
        moving.t += dt;
        if moving.t >= 1.0 {
            moving.t = 1.0;
            moving.forward = false;
        }
    } else {
        moving.t -= dt;
        if moving.t <= 0.0 {
            moving.t = 0.0;
            moving.forward = true;
        }
    }
}

/// Compute midpoint-centered, Y-flipped start and end positions
fn centered_start_end(moving: &Moving, map_height: f32) -> (Vec2, Vec2) {
    let start = Vec2::new(
        moving.start_x as f32,
        map_height - moving.start_y as f32,
    );
    let end = Vec2::new(
        moving.end_x as f32,
        map_height - moving.end_y as f32,
    );

    let midpoint = (start + end) / 2.0;
    let start_centered = start - midpoint;
    let end_centered = end - midpoint;

    (start_centered + midpoint, end_centered + midpoint)
}

/// Standard cubic-bezier evaluation (De Casteljau)
fn cubic_bezier_y(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    uuu * 0.0 +
    3.0 * uu * t * y1 +
    3.0 * u * tt * y2 +
    ttt * 1.0
}

/// Apply a standard ease-in-out cubic-bezier (0.42, 0.0, 0.58, 1.0)
fn ease_in_out(t: f32) -> f32 {
    cubic_bezier_y(t, 0.42, 0.0, 0.58, 1.0)
}

fn ease_kick(t: f32) -> f32 {
    cubic_bezier_y(t, 0.68, -0.55, 0.265, 1.55)
}

pub fn move_platforms_with_moving(
    time: Res<Time>,
    map: Res<MapFile>,
    mut q: Query<(&mut Transform, &mut EntityAttrs)>,
) {
    let map_height = (map.metadata.rows * map.metadata.tile_size_px) as f32;

    for (mut transform, mut attrs) in &mut q {
        if let Some(moving) = &mut attrs.moving {
            let dt = time.delta_secs() * moving.speed;

            // Update t with ping-pong behavior
            update_t(moving, dt);

            // Compute start & end positions in world space (centered + flipped)
            let (start, end) = centered_start_end(moving, map_height);

            // Apply easing
            let eased_t = ease_in_out(moving.t);

            // Interpolate & apply to transform
            let pos = start.lerp(end, eased_t);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}


// move a half screen right and a half screen up.
// so that the origin is in the positive coordinate system
fn init_player_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 0, // draw first (background/world)
            ..default()
        },
        Transform {
            translation: Vec3::new(
                 SCREEN.0 / 2.0,
                 SCREEN.1 / 2.0,
                 0.0,
             ),
             ..Default::default() 
        },
        RenderLayers::layer(0),
        MainCamera,
    ));
}

pub fn run() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerSpawnPoint { position: PLAYER_INITIAL_POSITION })
        .insert_resource(PlayerSpawnVelocity { velocity: PLAYER_INITIAL_VELOCITY })

        .add_systems(Startup, (load_map_resouces, load_map).chain())
        .add_systems(Startup, init_player_camera)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(DevModePlugin)

        .add_systems(Update, update_camera)
        .add_systems(Update, move_platforms_with_moving)
        .run();
}

// Camera Components
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FollowedPlayer;

const CAMERA_DECAY_RATE: f32 = 3.;

// System for the camera movement
fn update_camera(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<FollowedPlayer>)>,
    player: Single<&Transform, (With<FollowedPlayer>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { y, .. } = player.translation;

    let min_y = SCREEN.1 / 2.0;
    let clamped_y = y.max(min_y);
    let target = Vec3::new(camera.translation.x, clamped_y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());
}
