use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Controlled;
impl Component for Controlled {
    type Storage = NullStorage<Self>;
}