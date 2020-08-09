use amethyst::{
    SimpleState,
    StateData,
    GameData,
    core::ecs::{World, WorldExt, Builder},
    core::Transform
};
use crate::components::Player;

pub struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        init_player(world);
    }
}

fn init_player(world: &mut World) {
    world
        .create_entity()
        .with(Transform::default())
        .with(Player)
        .build();
}
