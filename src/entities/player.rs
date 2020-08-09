use amethyst::{
    core::Transform,
    ecs::prelude::*,
};

use crate::components::{Player, Controlled, Direction, Speed};

pub fn init_player(world: &mut World, speed: f32) -> Entity {
    world
        .create_entity()
        .with(Transform::default())
        .with(Player)
        .with(Controlled)
        .with(Direction::default())
        .with(Speed(speed))
        .build()
}