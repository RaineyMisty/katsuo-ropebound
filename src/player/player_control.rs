// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>

use bevy::prelude::*;
use crate::config::physics::{
    PLAYER_MOVE_FORCE,
    PLAYER_JUMP_FORCE,
    PLAYER_CONTROL_SPEED_LIMIT,
};
use crate::player::bundle::Player;
use crate::components::motion::{
    ControlForce,
    GroundState,
    JumpController,
    NetForce,
    Velocity,
};

/// Discrete per-frame input state for one player entity.
#[derive(Event)]
pub struct PlayerInputEvent {
    pub entity: Entity,
    pub left: bool,
    pub right: bool,
    pub jump_pressed: bool,
    pub jump_just_released: bool,
}

/// Runs during `FixedUpdate`, applies forces based on latest input events.
pub fn player_movement_input_system(
    time: Res<Time>,
    mut reader: EventReader<PlayerInputEvent>,
    mut query: Query<(
        &mut Velocity,
        &mut ControlForce,
        &mut NetForce,
        &mut JumpController,
        &mut GroundState,
    )>,
) {
    for event in reader.read() {
        if let Ok((
            velocity,
            mut control_force,
            mut net_force,
            mut jump_controller,
            ground_state,
        )) = query.get_mut(event.entity)
        {
            control_force.0.y = 0.0;

            apply_horizontal_movement(&velocity, &mut control_force, event);

            apply_jump(&time, &mut control_force, &mut jump_controller, &ground_state, event);

            net_force.0 += control_force.0;
        }
    }
}

/// Collects keyboard input every `Update` frame and emits `PlayerInputEvent`s.
/// This ensures we never miss `just_released` frames.
pub fn player_input_collection_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &Player)>,
    mut writer: EventWriter<PlayerInputEvent>,
) {
    for (entity, player) in &query {
        writer.write(PlayerInputEvent {
            entity,
            left: keyboard_input.pressed(player.controls.left),
            right: keyboard_input.pressed(player.controls.right),
            jump_pressed: keyboard_input.pressed(player.controls.up),
            jump_just_released: keyboard_input.just_released(player.controls.up),
        });
    }
}

fn apply_horizontal_movement(
    velocity: &Velocity,
    control_force: &mut ControlForce,
    event: &PlayerInputEvent,
) {
    // reset horizontal force every frame
    control_force.0.x = 0.0;

    let resistance = PLAYER_MOVE_FORCE / PLAYER_CONTROL_SPEED_LIMIT;
    let resistance_force = resistance * velocity.0.x.abs();

    if event.left {
        if velocity.0.x > -PLAYER_CONTROL_SPEED_LIMIT {
            control_force.0.x = -PLAYER_MOVE_FORCE;
            if velocity.0.x < 0.0 {
                control_force.0.x += resistance_force;
            }
        }
    }

    if event.right {
        if velocity.0.x < PLAYER_CONTROL_SPEED_LIMIT {
            control_force.0.x = PLAYER_MOVE_FORCE;
            if velocity.0.x > 0.0 {
                control_force.0.x -= resistance_force;
            }
        }
    }
}

fn apply_jump(
    time: &Time,
    control_force: &mut ControlForce,
    jump_controller: &mut JumpController,
    ground_state: &GroundState,
    event: &PlayerInputEvent,
) {
    let can_jump = ground_state.is_grounded || !ground_state.coyote_timer.finished();

    // Start jump on press
    if event.jump_pressed && !jump_controller.is_jumping && can_jump {
        control_force.0.y = PLAYER_JUMP_FORCE;
        jump_controller.is_jumping = true;
        jump_controller.jump_time_elapsed = 0.0;
    }

    // While holding, apply extra force until max duration
    if jump_controller.is_jumping
        && event.jump_pressed
        && jump_controller.jump_time_elapsed < jump_controller.max_jump_duration
    {
        jump_controller.jump_time_elapsed += time.delta_secs();
        control_force.0.y += PLAYER_JUMP_FORCE * jump_controller.jump_multiplier;
    }

    // Stop jumping if button released OR jump duration expired
    if jump_controller.is_jumping
        && (event.jump_just_released
            || jump_controller.jump_time_elapsed >= jump_controller.max_jump_duration)
    {
        jump_controller.is_jumping = false;
    }
}

