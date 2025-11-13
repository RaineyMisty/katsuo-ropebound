// SPDX-License-Identifier: MIT
// Copyright (c) 2025
// Author:
// Description: <CoinPlugin>
use bevy::prelude::*;

mod spawn;

use self::spawn::coin_spawn;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, coin_spawn);
        //    .add_systems(Update, coin_detect);
    }
}