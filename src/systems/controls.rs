use amethyst::{
    core::{Time, Transform},
    ecs::prelude::*,
    input::InputHandler,
};

use crate::{
    components::{Controlled, Speed},
    input_bindings::{AxisBinding, InputBindings},
};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        Read<'s, InputHandler<InputBindings>>,
        Read<'s, Time>,
        ReadStorage<'s, Controlled>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (input, time, controlled, speeds, mut locals): Self::SystemData) {
        if let Some((dx, dy)) = get_delta(&input) {
            let (mut dy, mut dx) = dy.atan2(dx).sin_cos();
            let sec = time.delta_seconds();
            dx *= sec;
            dy *= sec;
            for (_, &Speed(speed), local) in (&controlled, &speeds, &mut locals).join() {
                let x = dx * speed;
                let y = dy * speed;
                local.append_translation_xyz(x, y, 0.);
            }
        }
    }
}

fn get_delta(input: &InputHandler<InputBindings>) -> Option<(f32, f32)> {
    let dx = input
        .axis_value(&AxisBinding::Horizontal)
        .unwrap_or_default();
    let dy = input.axis_value(&AxisBinding::Vertical).unwrap_or_default();
    if dx != 0. || dy != 0. {
        Some((dx, dy))
    } else {
        None
    }
}
