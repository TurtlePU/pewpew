use amethyst::{
    core::ecs::{System, Read},
    input::InputHandler,
    core::ecs::{WriteStorage, ReadStorage, Join},
    core::Transform
};
use crate::{
    input_bindings::{InputBindings, AxisBinding},
    components::Player
};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        Read<'s, InputHandler<InputBindings>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, (input, mut local, player): Self::SystemData) {
        (|| -> Option<()> {
            let dx = input.axis_value(&AxisBinding::Horizontal)?;
            let dy = input.axis_value(&AxisBinding::Vertical)?;
            for (local, _) in (&mut local, &player).join() {
                local.append_translation_xyz(dx, dy, 0.);
            }
            Some(())
        })();
    }
}