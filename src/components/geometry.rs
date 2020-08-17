use std::ops::Deref;

use amethyst::ecs::{Component, VecStorage};
use ncollide2d::shape::Shape;

pub struct Geometry(pub Box<dyn Shape<f32>>);

impl Component for Geometry {
    type Storage = VecStorage<Self>;
}

impl<T> From<T> for Geometry
where
    T: Shape<f32>,
{
    fn from(shape: T) -> Self {
        Self(Box::new(shape))
    }
}

impl Deref for Geometry {
    type Target = dyn Shape<f32>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
