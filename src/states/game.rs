use amethyst::{
    SimpleState,
    StateData,
    GameData,
    core::ecs::WorldExt
};
use crate::{
    components::Player,
    entities::{init_player, init_camera},
    config::PewPewConfig,
};

pub struct GameState {
    config: PewPewConfig,
}

impl GameState {
    pub fn new(config: PewPewConfig) -> Self {
        Self { config }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.register::<Player>();
        init_player(world);
        init_camera(world, &self.config.camera_fov);
    }
}