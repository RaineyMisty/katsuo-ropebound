// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Xiaoting Wang
// Author: Xiaoting Wang <xiw323@pitt.edu>
// Description: <Camera Resource>
use bevy::prelude::*;

#[derive(Resource, Default)]
pub(super) struct FollowTarget( Option<Entity> );