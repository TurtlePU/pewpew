use amethyst::core::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct HitBox;

impl Component for HitBox {
    type Storage = NullStorage<Self>;
}
