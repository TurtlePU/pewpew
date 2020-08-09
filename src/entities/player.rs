use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{Controlled, Direction, Player, Speed};

pub fn init_player(world: &mut World, speed: f32, sprite_sheet: Handle<SpriteSheet>) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Transform::default())
        .with(Player)
        .with(Controlled)
        .with(Direction::default())
        .with(Speed(speed))
        .build()
}
