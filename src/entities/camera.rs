use amethyst::{
    core::{Transform, Parent},
    ecs::prelude::*,
    renderer::Camera,
};

pub fn init_camera(world: &mut World, follows: Entity, fov: &(f32, f32)) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_z(1.);

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(fov.0, fov.1))
        .with(Parent { entity: follows })
        .build()
}