use amethyst::{ecs::prelude::*, input::InputHandler};

use crate::{
    components::{Controlled, Direction},
    input_bindings::{AxisBinding, InputBindings},
};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        Read<'s, InputHandler<InputBindings>>,
        ReadStorage<'s, Controlled>,
        WriteStorage<'s, Direction>,
    );

    fn run(&mut self, (input, controlled, mut directions): Self::SystemData) {
        if let Some((dx, dy)) = get_delta(&input) {
            let angle = dy.atan2(dx);
            for (_, direction) in (&controlled, &mut directions).join() {
                direction.angle = Some(angle);
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
