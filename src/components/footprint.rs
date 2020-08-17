use amethyst::{
    core::math::{ComplexField, Isometry3, Vector3},
    ecs::prelude::*,
};
use ncollide2d::nalgebra::{Isometry2, Vector2};

#[derive(Clone)]
pub struct FootPrint(pub Isometry2<f32>);

impl Component for FootPrint {
    type Storage = VecStorage<Self>;
}

impl FootPrint {
    pub fn new(entity: Entity, counter: &mut impl FootPrintCounter) -> Self {
        Self::from(counter.count(entity))
    }
}

impl From<Isometry3<f32>> for FootPrint {
    fn from(iso: Isometry3<f32>) -> Self {
        let tl = iso.translation;
        let (_, _, angle) = iso.rotation.euler_angles();
        Self(Isometry2::new(Vector2::new(tl.x, tl.y), angle))
    }
}

impl Into<Isometry3<f32>> for &FootPrint {
    fn into(self) -> Isometry3<f32> {
        let tr = &self.0.translation;
        Isometry3::new(
            Vector3::new(tr.x, tr.y, 0.),
            Vector3::new(0., 0., self.0.rotation.argument()),
        )
    }
}

impl Default for FootPrint {
    fn default() -> Self {
        Self(Isometry2::identity())
    }
}

pub trait FootPrintCounter {
    fn count(&mut self, entity: Entity) -> Isometry3<f32>;
}
