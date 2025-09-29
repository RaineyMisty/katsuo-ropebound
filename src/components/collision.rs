
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Aabb {pub length:f32 ,pub width:f32} // L and width

impl Aabb{
    pub fn halfed(&self) -> Vec2 {
        return Vec2 :: new(self.length * 0.5, self.width *0.5);
    }
    pub fn min_max(&self, center: Vec2) -> (Vec2, Vec2) {
        let half = self.halfed();
        let min = center - half;
        let max = center + half;
        (min, max)
    }
}
