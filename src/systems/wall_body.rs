use std::cell::RefCell;

use amethyst::{
    core::{Parent, Transform},
    ecs::{prelude::*, shrev::EventChannel},
};

use crate::{
    components::{Body, FootPrint, Geometry, Wall},
    events::{FixTransformEvent, UpdateFootPrintEvent},
    geometry::{CachingFootPrintCounter, ToiObject},
};

pub struct WallBodySystem;

const EPS: f32 = 0.01;

impl<'s> System<'s> for WallBodySystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Geometry>,
        ReadStorage<'s, FootPrint>,
        ReadStorage<'s, Body>,
        ReadStorage<'s, Wall>,
        Write<'s, EventChannel<FixTransformEvent>>,
        Write<'s, EventChannel<UpdateFootPrintEvent>>,
    );

    fn run(
        &mut self,
        (
            entities,
            locals,
            parents,
            geometries,
            footprints,
            bodies,
            walls,
            mut fix_channel,
            mut update_channel,
        ): Self::SystemData,
    ) {
        let counter = RefCell::new(CachingFootPrintCounter::new(&locals, &parents));

        let min_toi = |body_toi: &ToiObject, entity: Entity| {
            (
                walls.mask() | bodies.mask(),
                &entities,
                &footprints,
                &geometries,
            )
                .join()
                .map(|(_, a, b, c)| (a, b, c))
                .filter(|(wall_entity, ..)| entity != *wall_entity)
                .map(|(entity, footprint, geometry)| {
                    ToiObject::build(entity, geometry, footprint, &mut *counter.borrow_mut())
                })
                .filter_map(|wall_toi| Some(body_toi.toi(&wall_toi)?.toi))
                .filter(|&toi| toi > 0.)
                .map(|toi| (toi - EPS).max(0.))
                .fold(1.0f32, |x, y| x.min(y))
        };

        let (footprints, fixes): (Vec<_>, Vec<_>) =
            (&bodies, &entities, &footprints, &geometries, &parents)
                .join()
                .map(|(_, entity, footprint, geometry, parent)| {
                    (
                        ToiObject::build(entity, geometry, footprint, &mut *counter.borrow_mut()),
                        entity,
                        parent,
                    )
                })
                .map(|(body_toi, entity, parent)| {
                    let time = min_toi(&body_toi, entity);
                    let (fix, new_foot) = body_toi.interpolate(time);
                    (
                        UpdateFootPrintEvent(entity, new_foot),
                        FixTransformEvent(parent.entity, fix),
                    )
                })
                .unzip();

        update_channel.iter_write(footprints.into_iter());
        fix_channel.iter_write(fixes.into_iter());
    }
}
