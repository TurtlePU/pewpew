use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    GameData, SimpleState, StateData,
};
use ncollide2d::shape::Ball;

use crate::{
    components::Player,
    config::PewPewConfig,
    entities::{init_camera, init_level_entity, init_player, init_player_body},
    level::LevelConfig,
};

pub struct GameState {
    config: PewPewConfig,
    level: LevelConfig,
}

impl GameState {
    pub fn new(config: PewPewConfig, level: LevelConfig) -> Self {
        Self { config, level }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.register::<Player>();
        let sprite_sheet = load_sprite_sheet(world);
        let player = init_player(world, self.config.player_speed, sprite_sheet.clone());
        let unit = self.config.unit_width;
        init_player_body(world, player, Ball::new(0.5 * unit));
        init_camera(world, player, &self.config.camera_fov);
        for entity in self.level.gen_level() {
            init_level_entity(world, sprite_sheet.clone(), unit, entity);
        }
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
