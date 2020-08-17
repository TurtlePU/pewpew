use amethyst::core::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Fragile;

impl Component for Fragile {
    type Storage = NullStorage<Self>;
}
