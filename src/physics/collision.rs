use crate::config::physics::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::PlayerCollider;
use bevy::math::bounding::{Aabb2d, AabbCast2d, BoundingVolume, RayCast2d};
use bevy::math::{Dir2, Ray2d};

use bevy::math::bounding::IntersectsVolume;
use crate::player::bundle::Player;
use bevy::{prelude::*, transform};
use crate::config::physics::GRAVITY;
use crate::components::collision::Aabb;

use crate::components::motion::{GroundState, Momentum, Velocity};
use crate::map::Collider;

const PLATFORM_FRICTION: f32 = 0.9;
const EPS: f32 = 0.05;              // contact offset ("skin")

/// Predict the player's AABB for the next frame
fn predicted_aabb(
    transform: &Transform,
    velocity: &Velocity,
    player_collider: &PlayerCollider,
    dt: f32,
) -> Aabb2d {
    let future_pos = transform.translation.truncate() + velocity.0 * dt;
    player_collider.aabb.translated_by(future_pos)
}

fn resolve_collision(
    player_pos: &mut Vec3,
    velocity: &mut Vec2,
    momentum: &mut Vec2,
    ground: &mut GroundState,
    offset: Vec2,
) {
    if offset.x.abs() > offset.y.abs() {
        // Horizontal collision
        velocity.x = 0.0;
        momentum.x = 0.0;
    } else {
        // Vertical collision
        // ✅ Instead of applying offset.y blindly, only correct penetration

        velocity.y = 0.0;
        momentum.y = 0.0;

        if offset.y > 0.0 {
            ground.is_grounded = true;
            velocity.x *= PLATFORM_FRICTION;
            momentum.x *= PLATFORM_FRICTION;
        }
    }
}



/// Main player–platform collision system
pub fn player_vs_collider_system(
    time: Res<Time>,
    mut players: Query<(
        &mut Transform,
        &mut Velocity,
        &mut Momentum,
        &PlayerCollider,
        &mut GroundState,
    ), With<Player>>,
    colliders: Query<(&Transform, &Collider), Without<Player>>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut velocity, mut momentum, player_collider, mut ground) in players.iter_mut() {
        let mut player_aabb = predicted_aabb(&transform, &velocity, player_collider, dt);
        ground.is_grounded = false;

        for (collider_transform, collider) in colliders.iter() {
            let collider_pos = collider_transform.translation.truncate();
            let collider_aabb = collider.aabb.translated_by(collider_pos);

            if player_aabb.intersects(&collider_aabb) {
                let mut player_pos = transform.translation;
                let player_center = player_aabb.center();
                let closest = collider_aabb.closest_point(player_center);
                let offset = player_center - closest;

                resolve_collision(
                    &mut player_pos,
                    &mut velocity.0,
                    &mut momentum.0,
                    &mut ground,
                    offset,
                );

                // Update the AABB after resolution
                player_aabb = player_collider.aabb.translated_by(player_pos.truncate());
            }
        }
    }
}


pub fn player_player_coll_system (
     time: Res<Time>,
        mut query:Query<(
            Entity,&mut Velocity,&mut Transform,&mut Momentum, &mut Aabb)>,
        ){

        let mut players_stuff: Vec<_> = query.iter_mut().collect();
        let change_intime = time.delta_secs();
        //iterating through players
        for i in 0..players_stuff.len(){
            if i + 1 >= players_stuff.len() {
                 break; 
                }
            let (one, two) = players_stuff.split_at_mut(i+1);
            //compare throug combinatorics
            if let Some(obj1) = one.last_mut() {
                // check 2nd obj
                for obj2 in two.iter_mut(){

                    let future1 = obj1.2.translation.truncate() + obj1.1.0 * change_intime;
                    let future2 = obj2.2.translation.truncate() + obj2.1.0 * change_intime;
                    // CHECK IF THEY HIT 
                    if check_aabb(future1, obj1.4.halfed(), future2, obj2.4.halfed()){
                        //think about this in a perfectly in elastic collision
                        let total_momentum = (obj1.3.0.x) + (obj2.3.0.x);
                        if obj1.3.0.x.abs() > obj2.3.0.x.abs(){
                            obj2.3.0.x = total_momentum*0.5;
                            obj1.3.0.x = total_momentum*0.5;
                            obj1.1.0.x = 0.;
                            //info!("This is a hit");
                        }
                        else if obj2.3.0.x.abs() > obj1.3.0.x.abs(){
                            obj1.3.0.x = obj2.3.0.x;
                            obj2.3.0.x = 0.;
                            obj2.1.0.x = 0.;
                        }
                        else{
                            obj1.1.0.x = 0.;
                            obj2.1.0.x = 0.;
                        }
                    }
                }
                
    }
}
        }

fn check_aabb(pos1: Vec2, width: Vec2, pos2: Vec2, width2: Vec2) -> bool{
    //possible future use for collision top and collision bottom
    let collisioned = (pos1.x - pos2.x).abs() <= width.x + width2.x && (pos1.y - pos2.y).abs() <= width.y + width2.y;
    //let collision_top = (pos1.y - pos2.y).abs() <= width.y + width2.y && (pos1.y - pos2.y).abs() <= width.y + width2.y;
    return collisioned;
}

fn check_top(pos1: Vec2, width: Vec2, pos2: Vec2, width2: Vec2) -> bool{
    return (pos1.x - pos2.x).abs() <= width.x + width2.x && (pos1.y > pos2.y || pos2.y > pos1.y) && (pos1.y - width.y) <= (pos2.y + width2.y);
}

pub fn update_coyote_timer_system(
    time: Res<Time>,
    mut query:Query<&mut GroundState, With<Player>>,
) {
    for mut ground_state in &mut query {
        // If in air tick the coyote timer
        if !ground_state.is_grounded {
            ground_state.coyote_timer.tick(time.delta());
        }
        // Grounded reset timer
        else {
            ground_state.coyote_timer.reset();
        }
    }
}
