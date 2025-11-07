// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope component>
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Rope;

#[derive(Clone, Copy, Debug)]
pub enum EndPoint {
    Body(Entity),
    Fixed(Vec2),
}

#[derive(Component, Debug)]
pub struct EndPoints {
    pub head: EndPoint,
    pub tail: EndPoint,
}