use crate::config::physics::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::bundle::Player;
use crate::components::motion::{Momentum, Velocity};
use bevy::{prelude::*, transform};
use crate::config::physics::GRAVITY;
use crate::components::collision::Aabb;
use crate::components::motion::{NetForce, Gravity, Mass};
use bevy::prelude::*;

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
