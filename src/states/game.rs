use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    ecs::prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    GameData, SimpleState, StateData,
};

use crate::{
    components::Player,
    config::PewPewConfig,
    entities::{init_camera, init_player},
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
        let sprite_sheet = load_sprite_sheet(world);
        let player = init_player(world, self.config.player_speed, sprite_sheet.clone());
        init_camera(world, player, &self.config.camera_fov);
        draw_square(world, sprite_sheet);
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

fn draw_square(world: &mut World, sprite_sheet: Handle<SpriteSheet>) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };
    world
        .create_entity()
        .with(Transform::default())
        .with(sprite_render)
        .build()
}
