use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};
use ncollide2d::{nalgebra::Vector2, shape::Cuboid};

use crate::{
    components::Geometry,
    level::{LevelEntity, LevelEntityKind::*},
};

pub fn init_level_entity(
    world: &mut World,
    sprite_sheet: Handle<SpriteSheet>,
    unit: f32,
    entity: LevelEntity,
) -> Entity {
    let sprite_number = match entity.kind {
        Wall => 1,
        Exit => 2,
    };
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number,
    };
    let mut transform = Transform::default();
    let x = entity.pos.0 as f32 * unit;
    let y = entity.pos.1 as f32 * unit;
    transform.set_translation_xyz(x, y, 0.);
    let mut builder = world.create_entity().with(transform).with(sprite_render);
    if let Wall = entity.kind {
        builder = builder
            .with(crate::components::Wall)
            .with(Geometry::from(Cuboid::new(
                (0.5 * unit - 1.) * Vector2::new(1., 1.),
            )));
    }
    builder.build()
}
