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

    pub fn intersects(&self, pos_a: Vec2, other: &Collider, pos_b: Vec2) -> bool {
        let (min_a, max_a) = self.min_max(pos_a);
        let (min_b, max_b) = other.min_max(pos_b);
        min_a.x < max_b.x && max_a.x > min_b.x &&
        min_a.y < max_b.y && max_a.y > min_b.y
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

impl GameEntityBundle {
    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn(self);
    }
}
