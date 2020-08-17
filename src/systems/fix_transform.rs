use amethyst::{
    core::{
        math::ComplexField,
        shrev::{EventChannel, ReaderId},
        SystemDesc, Transform,
    },
    ecs::prelude::*,
};

use crate::events::FixTransformEvent;

pub struct FixTransformDesc;

impl SystemDesc<'_, '_, FixTransformSystem> for FixTransformDesc {
    fn build(self, world: &mut World) -> FixTransformSystem {
        <FixTransformSystem as System>::SystemData::setup(world);
        FixTransformSystem(world.fetch_mut::<EventChannel<_>>().register_reader())
    }
}

pub struct FixTransformSystem(ReaderId<FixTransformEvent>);

impl<'s> System<'s> for FixTransformSystem {
    type SystemData = (
        Read<'s, EventChannel<FixTransformEvent>>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (events, mut locals): Self::SystemData) {
        for FixTransformEvent(entity, fix) in events.read(&mut self.0) {
            let tf = locals
                .entry(*entity)
                .unwrap()
                .or_insert_with(Transform::default);
            tf.append_rotation_z_axis(fix.rotation.argument());
            let tl = &fix.translation.vector;
            tf.append_translation_xyz(tl.x, tl.y, 0.);
        }
    }
}
