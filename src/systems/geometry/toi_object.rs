use amethyst::ecs::Entity;
use ncollide2d::{
    interpolation::{InterpolatedRigidMotion, RigidMotion},
    query::{self, TOI},
    shape::Shape,
};

use crate::components::{footprint::FootPrintCounter, FootPrint, Geometry};

pub struct ToiObject<'s> {
    shape: &'s dyn Shape<f32>,
    motion: InterpolatedRigidMotion<f32>,
}

impl<'s> ToiObject<'s> {
    pub fn new(geometry: &'s Geometry, footprint: &FootPrint, new_footprint: FootPrint) -> Self {
        Self {
            shape: &**geometry,
            motion: InterpolatedRigidMotion::new(footprint.0.clone(), new_footprint.0),
        }
    }

    pub fn build(
        entity: Entity,
        geometry: &'s Geometry,
        footprint: &FootPrint,
        counter: &mut impl FootPrintCounter,
    ) -> Self {
        Self::new(geometry, footprint, FootPrint::new(entity, counter))
    }

    pub fn interpolate(&self, time: f32) -> FootPrint {
        FootPrint(self.motion.position_at_time(time))
    }

    pub fn toi(&self, rhs: &ToiObject<'_>) -> Option<TOI<f32>> {
        query::nonlinear_time_of_impact(&self.motion, self.shape, &rhs.motion, rhs.shape, 1., 0.)
    }
}
