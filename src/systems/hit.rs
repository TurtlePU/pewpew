use std::cell::RefCell;

use amethyst::{
    core::{Parent, Transform},
    ecs::prelude::*,
    shrev::EventChannel,
};

use crate::{
    components::{FootPrint, Geometry, HitBox, Hitter},
    events::{HitEvent, UpdateFootPrintEvent},
    geometry::{CachingFootPrintCounter, ToiObject},
};

pub struct HitSystem;

impl<'s> System<'s> for HitSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Geometry>,
        ReadStorage<'s, FootPrint>,
        ReadStorage<'s, Hitter>,
        ReadStorage<'s, HitBox>,
        Write<'s, EventChannel<HitEvent>>,
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
            hitters,
            hitboxes,
            mut hit_channel,
            mut update_channel,
        ): Self::SystemData,
    ) {
        let counter = RefCell::new(CachingFootPrintCounter::new(&locals, &parents));

        let hits = |hitter_toi: &ToiObject| {
            (&hitboxes, &entities, &geometries, &footprints, &parents)
                .join()
                .map(|(_, entity, geometry, footprint, parent)| {
                    (
                        ToiObject::build(entity, geometry, footprint, &mut *counter.borrow_mut()),
                        parent.entity,
                    )
                })
                .filter_map(|(hitbox_toi, owner)| Some((hitter_toi.toi(&hitbox_toi)?.toi, owner)))
                .filter(|(toi, ..)| *toi > 0.)
                .fold((1.0f32, vec![]), |(min_toi, mut owners), (toi, owner)| {
                    if toi < min_toi {
                        (toi, vec![owner])
                    } else if toi == min_toi {
                        owners.push(owner);
                        (toi, owners)
                    } else {
                        (min_toi, owners)
                    }
                })
                .1
        };

        let (hits, updates): (Vec<_>, Vec<_>) = (&hitters, &entities, &geometries, &footprints)
            .join()
            .map(|(_, entity, geometry, footprint)| {
                (
                    ToiObject::build(entity, geometry, footprint, &mut *counter.borrow_mut()),
                    entity,
                )
            })
            .map(|(hitter_toi, hitter)| (hitter_toi.end(), hitter, hits(&hitter_toi)))
            .map(|(end, hitter, hits)| {
                let hit_events: Vec<_> = hits
                    .into_iter()
                    .map(|hitbox| HitEvent { hitter, hitbox })
                    .collect();
                (hit_events, UpdateFootPrintEvent(hitter, end))
            })
            .unzip();

        let hits: Vec<_> = hits.into_iter().flatten().collect();

        update_channel.iter_write(updates.into_iter());
        hit_channel.iter_write(hits.into_iter());
    }
}
