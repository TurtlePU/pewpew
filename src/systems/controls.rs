use amethyst::{
    ecs::prelude::*,
    input::InputHandler,
};

use crate::{
    input_bindings::{InputBindings, AxisBinding},
    components::{Controlled, Direction}
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
    let dx = input.axis_value(&AxisBinding::Horizontal);
    let dy = input.axis_value(&AxisBinding::Vertical);
    match (dx, dy) {
        (Some(dx), dy) => Some((dx, dy.unwrap_or_default())),
        (dx, Some(dy)) => Some((dx.unwrap_or_default(), dy)),
        _ => None,
    }
}