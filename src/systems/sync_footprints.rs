use crate::{
    components::FootPrint, events::UpdateFootPrintEvent, geometry::CachingFootPrintCounter,
};
use amethyst::{
    core::{Parent, SystemDesc, Transform},
    ecs::{prelude::*, shrev::EventChannel},
};

pub struct SyncFootPrintsDesc;

impl SystemDesc<'_, '_, SyncFootprintsSystem> for SyncFootPrintsDesc {
    fn build(self, world: &mut World) -> SyncFootprintsSystem {
        <SyncFootprintsSystem as System>::SystemData::setup(world);
        SyncFootprintsSystem(world.fetch_mut::<EventChannel<_>>().register_reader())
    }
}

pub struct SyncFootprintsSystem(ReaderId<UpdateFootPrintEvent>);

impl<'s> System<'s> for SyncFootprintsSystem {
    type SystemData = (
        Read<'s, EventChannel<UpdateFootPrintEvent>>,
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Parent>,
        WriteStorage<'s, FootPrint>,
    );

    fn run(&mut self, (channel, entities, locals, parents, mut footprints): Self::SystemData) {
        let counter = &mut CachingFootPrintCounter::new(&locals, &parents);
        for (_, entity) in (locals.mask() | parents.mask(), &entities).join() {
            footprints
                .entry(entity)
                .unwrap()
                .or_insert_with(|| FootPrint::new(entity, counter));
        }
        for UpdateFootPrintEvent(entity, footprint) in channel.read(&mut self.0) {
            footprints
                .insert(entity.clone(), footprint.clone())
                .unwrap();
        }
    }
}
