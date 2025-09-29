use bevy::prelude::*;

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
}

#[derive(Bundle)]
pub struct BaseComponents {
    sprite: Sprite,
    transform: Transform,
    visibility: Visibility,
    name: Name,
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
/// Shared handle store for building entities.
/// This replaces the "God object" factory.
#[derive(Resource, Clone)]
pub struct EntityFactory {
    pub image: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

impl EntityFactory {
    /// Creates a new entity builder with shared factory data.
    pub fn builder(&self) -> EntityBuilder {
        EntityBuilder::new(self)
    }
}

/// builder for game entities based on shared atlas/sprite data.
/// Example:
/// ```
/// commands.spawn(factory.builder()
///     .id("coin_1")
///     .index(5)
///     .position(Vec3::new(100.0, 200.0, 0.0))
///     .coin());
/// ```
pub struct EntityBuilder<'a> {
    factory: &'a EntityFactory,
    id: String,
    index: usize,
    position: Vec3,
    collider: Option<Collider>,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(factory: &'a EntityFactory) -> Self {
        Self {
            factory,
            id: "entity".to_string(),
            index: 0,
            position: Vec3::ZERO,
            collider: None,
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn collider(mut self, width: f32, height: f32, offset: Vec2) -> Self {
        self.collider = Some(Collider::new(width, height, offset));
        self
    }


    /// Builds a `PlatformBundle` entity.
    pub fn make_bundle(self) -> GameEntityBundle {
        GameEntityBundle {
            base: self.base_components(),
            collider: self.collider.unwrap_or(Collider::new(0.0, 0.0, Vec2::new(0.0,0.0))),
        }
    }

    fn base_components(&self) -> BaseComponents {
        BaseComponents {
            sprite: self.make_sprite(),
            transform: Transform::from_translation(self.position),
            visibility: Visibility::default(),
            name: Name::new(self.id.clone()),
        }
    }

    fn make_sprite(&self) -> Sprite {
        Sprite {
            image: self.factory.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: self.factory.atlas_layout.clone(),
                index: self.index,
            }),
            ..default()
        }
    }
}
