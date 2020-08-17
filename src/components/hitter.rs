use amethyst::core::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Hitter;

impl Component for Hitter {
    type Storage = NullStorage<Self>;
}
