use amethyst::core::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Body;

impl Component for Body {
    type Storage = NullStorage<Self>;
}
