use amethyst::ecs::{Component, VecStorage};

pub struct Speed(pub f32);

impl Component for Speed {
    type Storage = VecStorage<Self>;
}
