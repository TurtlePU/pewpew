use std::collections::BTreeMap;

use amethyst::{
    core::{math::Isometry3, Parent, Transform},
    ecs::{Entity, ReadStorage},
};

use crate::components::footprint::FootPrintCounter;

pub struct CachingFootPrintCounter<'s, 't> {
    cache: BTreeMap<Entity, Isometry3<f32>>,
    transform: &'t ReadStorage<'s, Transform>,
    parent: &'t ReadStorage<'s, Parent>,
}

impl<'s, 't> CachingFootPrintCounter<'s, 't> {
    pub fn new(
        transform: &'t ReadStorage<'s, Transform>,
        parent: &'t ReadStorage<'s, Parent>,
    ) -> Self {
        Self {
            cache: BTreeMap::new(),
            transform,
            parent,
        }
    }

    fn count_impl(&mut self, entity: Entity) -> Isometry3<f32> {
        let local = self.local_iso(entity);
        self.parent_iso(entity)
            .and_then(|parent| Some(parent * local?))
            .or(local)
            .unwrap_or_else(Isometry3::<f32>::identity)
    }

    fn local_iso(&mut self, entity: Entity) -> Option<Isometry3<f32>> {
        Some(self.transform.get(entity)?.isometry().clone())
    }

    fn parent_iso(&mut self, entity: Entity) -> Option<Isometry3<f32>> {
        Some(self.count(self.parent.get(entity)?.entity))
    }
}

impl FootPrintCounter for CachingFootPrintCounter<'_, '_> {
    fn count(&mut self, entity: Entity) -> Isometry3<f32> {
        if let Some(result) = self.cache.get(&entity) {
            Isometry3::clone(result)
        } else {
            let result = self.count_impl(entity);
            self.cache.insert(entity, result.clone());
            result
        }
    }
}
