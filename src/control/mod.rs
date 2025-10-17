// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Systems for player control>
use bevy::prelude::*

mod control;
mod intent;
mod spec_load;

mod component;

use self::control::keyboard_control_system;
use self::intent::scheme_to_intent_writer;
use self::spec_load::on_request_control;

use crate::event::RequestControl;
use crate::event::PlayerIntentEvent;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self app: mut App) {
        app.add_event<RequestControl>()
        .add_event<PlayerIntentEvent>()
        .add_systems(Update, on_request_control)
        .add_systems(Update, keyboard_control_system)
        .add_systems(Update, scheme_to_intent_writer)
    }
}