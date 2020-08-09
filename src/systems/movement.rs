use crate::components::{Direction, Speed};

use amethyst::{
    core::{Time, Transform},
    ecs::prelude::*,
};

#[derive(Default)]
pub struct MovementSystem {
    dirty: BitSet,
    reader: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (time, directions, speeds, mut locals): Self::SystemData) {
        self.dirty.clear();
        let events = directions.channel().read(self.reader.as_mut().unwrap());
        for event in events {
            if let ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) = event {
                self.dirty.add(*id);
            }
        }
        for (_, angle, speed, local) in (&self.dirty, &directions, &speeds, &mut locals).join() {
            if let Some(angle) = angle.angle {
                // FIXME: collisions, etc.
                let (sin, cos) = angle.sin_cos();
                let abs = speed.0 * time.delta_seconds();
                local.append_translation_xyz(abs * cos, abs * sin, 0.);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(WriteStorage::<Direction>::fetch(world).register_reader());
    }
}
