use crate::components::FootPrint;
use amethyst::core::ecs::Entity;

pub struct UpdateFootPrintEvent(pub Entity, pub FootPrint);
