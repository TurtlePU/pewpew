use amethyst::ecs::Entity;
use ncollide2d::nalgebra::Isometry2;

pub struct FixTransformEvent(pub Entity, pub Isometry2<f32>);
