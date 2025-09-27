use crate::config::physics::{PLAYER_MOVE_FORCE, PLAYER_JUMP_FORCE, PLAYER_CONTROL_SPEED_LIMIT};
use crate::player::bundle::Player;
use crate::components::motion::{Momentum, Velocity};
use bevy::{prelude::*, transform};
use crate::config::physics::GRAVITY;
use crate::components::collision::Aabb;
//use crate::components::motion::{NetForce, Gravity, Mass};

pub fn player_player_coll_system (
     time: Res<Time>,
        mut query:Query<(
            &mut Velocity,&mut Transform,&mut Momentum, &mut Aabb)>,
        ){

        let players_stuff = query.iter_mut();
        //iterating through player
        for i in 0..players_stuff.len(){

            for j in (i + 1)..players_stuff.len() {
                let obj1 = &players_stuff.take(i);
                let obj2 = &players_stuff.take(j);

                let player_half = player_col.size * 0.5;

                // Apply velocity
                player_tf.translation.x += velocity.x * time.delta_secs();
                player_tf.translation.y += velocity.y * time.delta_secs();

                //let plat_half = plat_col.size * 0.5;

                let player_min = player_tf.translation.truncate() - player_half;
                let player_max = player_tf.translation.truncate() + player_half;

                let plat_min = plat_tf.translation.truncate() - plat_half;
                let plat_max = plat_tf.translation.truncate() + plat_half;

                let overlap_x = player_min.x < plat_max.x && player_max.x > plat_min.x;
                let overlap_y = player_min.y < plat_max.y && player_max.y > plat_min.y;

                if overlap_x && overlap_y {
                // Compute penetration depths
                let pen_left = player_max.x - plat_min.x;
                let pen_right = plat_max.x - player_min.x;
                let pen_x = pen_left.min(pen_right);

                let pen_down = player_max.y - plat_min.y;
                let pen_up = plat_max.y - player_min.y;
                let pen_y = pen_down.min(pen_up);

                // Resolve along smaller penetration axis
                if pen_x < pen_y {
                    // ✅ Resolve X
                    if pen_left < pen_right {
                        player_tf.translation.x -= pen_left;
                    } else {
                        player_tf.translation.x += pen_right;
                    }
                    velocity.x = 0.0;
                } else {
                    // ✅ Resolve Y — but only if falling
                    if velocity.y <= 0.0 {
                        // Landing
                        player_tf.translation.y = plat_max.y + player_half.y;
                        grounded.0 = true;
                        velocity.y = 0.0;
                        jump_state.jump_count = 0;
                        jump_state.coyote_timer = 0.1;
                    } else {
                        // 🚫 If jumping, ignore vertical correction
                        // (prevents teleport-down bug)
                    }
                }
            }

        // Tick down coyote timer if not grounded
        if !grounded.0 {
            jump_state.coyote_timer -= time.delta_secs();
        }
    }
}
        }