use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    GameData, SimpleState, StateData,
};

use crate::{
    components::Player,
    config::PewPewConfig,
    entities::{init_camera, init_level_entity, init_player},
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
        init_camera(world, player, &self.config.camera_fov);
        for entity in self.level.gen_level() {
            init_level_entity(world, sprite_sheet.clone(), self.config.unit_width, entity);
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
