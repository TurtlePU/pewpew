use amethyst::{
    SimpleState,
    StateData,
    GameData,
    ecs::WorldExt,
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
        let player = init_player(world, 1.);
        init_camera(world, player, &self.config.camera_fov);
    }
}