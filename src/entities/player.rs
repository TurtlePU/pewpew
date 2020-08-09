use amethyst::{
    core::{
        ecs::{World, Entity, WorldExt, Builder},
        Transform
    }
};
use crate::components::{Player, Controlled};

pub fn init_player(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Transform::default())
        .with(Player)
        .with(Controlled)
        .build()
}