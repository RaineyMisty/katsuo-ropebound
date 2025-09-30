use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct MapTextureHandles {
    pub tile_fg: Handle<Image>,
    pub entity: Handle<Image>,
}

#[derive(Resource)]
pub struct AtlasLayoutResource {
    pub layout: Handle<TextureAtlasLayout>,
    pub indices: HashMap<String, usize>,
}

// Bevy data structs
#[derive(Component, Debug)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
}


impl Collider {
    pub fn new(width: f32, height: f32, offset: Vec2) -> Self {
        Self {
            size: Vec2::new(width, height),
            offset,
        }
    }

    pub fn min_max(&self, pos: Vec2) -> (Vec2, Vec2) {
        let min = pos + self.offset - self.size / 2.0;
        let max = pos + self.offset + self.size / 2.0;
        (min, max)
    }
    pub fn halfed(&self) -> Vec2 {
        self.size * 0.5
    }
}


#[derive(Bundle)]
pub struct BaseComponents {
    pub sprite: Sprite,
    pub transform: Transform,
    pub visibility: Visibility,
    pub name: Name,
}

#[derive(Bundle)]
pub struct GameEntityBundle {
    #[bundle()]
    pub base: BaseComponents,
    pub collider: Collider,
}
