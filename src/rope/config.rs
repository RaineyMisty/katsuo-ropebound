// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Tingxu Chen
// Author: Tingxu Chen <tic128@pitt.edu>
// Description: <Rope configuration>

// Rope parameters
pub const ROPE_REST_LENGTH: f32 = 5.0 * SCALE;  // in pixel
pub const ROPE_MAX_EXTENSION: f32 = 50.0;  // Maximum extension beyond rest length
pub const SPRING_CONSTANT: f32 = 80000.0;  // in Newton/pixel = kg/s^2