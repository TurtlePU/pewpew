use amethyst::{
    core::{
        Transform,
        ecs::{World, Entity, WorldExt, Builder}
    },
    renderer::Camera
};
use crate::components::Controlled;

pub fn init_camera(world: &mut World, fov: &(f32, f32)) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_z(1.);

    world
        .create_entity()
        .with(transform)
        .with(Controlled)
        .with(Camera::standard_2d(fov.0, fov.1))
        .build()
}