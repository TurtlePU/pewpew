use amethyst::{core::Parent, ecs::prelude::*};
use ncollide2d::shape::Shape;

use crate::components::{Body, Geometry};

pub fn init_player_body(world: &mut World, player: Entity, shape: impl Shape<f32>) -> Entity {
    world
        .create_entity()
        .with(Parent { entity: player })
        .with(Body)
        .with(Geometry::from(shape))
        .build()
}
