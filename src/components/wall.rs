use amethyst::core::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Wall;

impl Component for Wall {
    type Storage = NullStorage<Self>;
}
