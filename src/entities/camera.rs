use amethyst::{
    core::{Parent, Transform},
    ecs::prelude::*,
    renderer::Camera,
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
};

pub fn init_camera(world: &mut World, follows: Entity, fov: &(f32, f32)) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_z(1.);

    let ortho = CameraOrtho::new(
        CameraNormalizeMode::Contain,
        CameraOrthoWorldCoordinates {
            left: -0.5 * fov.0,
            right: 0.5 * fov.0,
            top: 0.5 * fov.1,
            bottom: -0.5 * fov.1,
        },
    );

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(fov.0, fov.1))
        .with(ortho)
        .with(Parent { entity: follows })
        .build()
}
