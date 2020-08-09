use amethyst::{
    core::ecs::{System, Read},
    input::InputHandler,
    core::ecs::{WriteStorage, ReadStorage, Join},
    core::Transform,
};
use crate::{
    input_bindings::{InputBindings, AxisBinding},
    components::Controlled,
};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        Read<'s, InputHandler<InputBindings>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Controlled>,
    );

    fn run(&mut self, (input, mut local, controlled): Self::SystemData) {
        let dx = input.axis_value(&AxisBinding::Horizontal).unwrap_or_default();
        let dy = input.axis_value(&AxisBinding::Vertical).unwrap_or_default();
        for (local, _) in (&mut local, &controlled).join() {
            local.append_translation_xyz(dx, dy, 0.);
        }
    }
}