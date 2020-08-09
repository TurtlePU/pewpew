use amethyst::ecs::prelude::*;

#[derive(Default)]
pub struct Direction {
    pub angle: Option<f32>,
}

impl Component for Direction {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}