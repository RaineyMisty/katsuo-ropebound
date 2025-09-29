use bevy::prelude::*;
use super::bundles::{GameEntityBundle, BaseComponents};
use super::Collider;

#[derive(Resource, Clone)]
pub struct EntityFactory {
    pub image: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

impl EntityFactory {
    /// Build a simple `GameEntityBundle` in one shot.
    ///
    /// `id`: name for the entity
    /// `index`: texture atlas index
    /// `position`: world position
    /// `collider`: optional (width, height) if you want to attach a Collider
    pub fn make_entity_bundle(
        &self,
        id: &str,
        index: usize,
        position: Vec3,
        collider: Option<(f32, f32)>,
    ) -> impl Bundle {
        // Build base sprite + transform bundle
        let base = BaseComponents {
            sprite: Sprite {
                image: self.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: self.atlas_layout.clone(),
                    index,
                }),
                ..Default::default()
            },
            transform: Transform::from_translation(position),
            visibility: Visibility::default(),
            name: Name::new(String::from(id)),
        };

        GameEntityBundle {
            base,
            collider: collider
                .map(|(w, h)| Collider::new(w, h, Vec2::ZERO))
                .unwrap_or_else(|| Collider::new(0.0, 0.0, Vec2::ZERO)),
        }
    }
}
