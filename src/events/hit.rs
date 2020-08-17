use amethyst::ecs::Entity;

pub struct HitEvent {
    pub hitter: Entity,
    pub hitbox: Entity,
}
