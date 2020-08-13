use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::level::{LevelEntity, LevelEntityKind::*};

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
    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .build()
}
