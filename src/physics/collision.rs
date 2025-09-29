use crate::config::physics::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::bundle::Player;
use bevy::{prelude::*, transform};
use crate::config::physics::GRAVITY;
use crate::components::collision::Aabb;

use bevy::prelude::*;
use crate::components::motion::{Velocity, Momentum};
use crate::map::Collider;

pub fn player_collider_collision_system(
    time: Res<Time>,
    // Moving entities (players)
    mut players: Query<(Entity, &mut Velocity, &mut Transform, &mut Momentum, &Aabb), With<Player>>,
    // Static colliders (platforms, walls)
    colliders: Query<(Entity, &Transform, &Collider), Without<Player>>,
) {
    let dt = time.delta_secs();

    for (player_entity, mut vel, mut trans, mut mom, aabb) in players.iter_mut() {
        let future_pos = trans.translation.truncate() + vel.0 * dt;

        for (collider_entity, collider_trans, collider) in colliders.iter() {
            let collider_pos = collider_trans.translation.truncate();

            if check_aabb(future_pos, aabb.halfed(), collider_pos, collider.halfed()) {
                // Compute overlaps for positional correction
                let (min_a, max_a) = aabb.min_max(future_pos);
                let (min_b, max_b) = collider.min_max(collider_pos);

                let overlap_x = (max_a.x - min_b.x).min(max_b.x - min_a.x);
                let overlap_y = (max_a.y - min_b.y).min(max_b.y - min_a.y);

                info!(
                    "💥 Player {:?} collided with Collider {:?} (overlap_x={}, overlap_y={})",
                    player_entity, collider_entity, overlap_x, overlap_y
                );

                if overlap_x < overlap_y {
                    if future_pos.x < collider_pos.x {
                        trans.translation.x -= overlap_x;
                    } else {
                        trans.translation.x += overlap_x;
                    }
                    vel.0.x = 0.0;
                    mom.0.x = 0.0;
                } else {
                    if future_pos.y < collider_pos.y {
                        trans.translation.y -= overlap_y;
                    } else {
                        trans.translation.y += overlap_y;
                    }
                    vel.0.y = 0.0;
                    mom.0.y = 0.0;
                }
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
                            info!("we hit");
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
