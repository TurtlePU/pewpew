use crate::input_bindings::InputBindings;

use amethyst::{
    core::Transform, ecs::prelude::*, input::InputHandler, utils::ortho_camera::CameraOrtho,
    window::ScreenDimensions,
};

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        Read<'s, InputHandler<InputBindings>>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, CameraOrtho>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (input, dim, orthos, mut locals): Self::SystemData) {
        let cx = 0.5 * dim.width();
        let cy = 0.5 * dim.height();
        let (x, y) = input.mouse_position().unwrap_or((cx, cy));
        for (ortho, local) in (&orthos, &mut locals).join() {
            let rx = ortho.world_coordinates.width() / dim.width();
            let ry = ortho.world_coordinates.height() / dim.height();
            local.set_translation_xyz(rx * (x - cx), ry * (cy - y), 1.);
        }
    }
}
