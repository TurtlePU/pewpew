use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir, ortho_camera::CameraOrthoSystem},
};

use crate::{
    config::PewPewConfig, input_bindings::InputBindings, level::LevelConfig, states::GameState,
    systems::*,
};

mod components;
mod config;
mod entities;
mod events;
mod geometry;
mod input_bindings;
mod level;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let pew_config = PewPewConfig::load(config_dir.join("config.ron"))?;

    let input_bundle = InputBundle::<InputBindings>::new()
        .with_bindings_from_file(config_dir.join("bindings.ron"))?;

    let render_to_window =
        RenderToWindow::from_config_path(display_config_path)?.with_clear(pew_config.bg_color);

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(render_to_window)
        .with_plugin(RenderFlat2D::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(rendering_bundle)?
        .with_bundle(TransformBundle::new())?
        .with(CameraOrthoSystem, "camera_ortho_system", &[])
        .with(ControlSystem, "control_system", &["input_system"])
        .with(CameraSystem, "camera_system", &["input_system"])
        .with(WallBodySystem, "wall_body_system", &["control_system"])
        .with(HitSystem, "hit_system", &["control_system"])
        .with_system_desc(
            SyncFootPrintsDesc,
            "sync_footprints_system",
            &["wall_body_system", "hit_system"],
        )
        .with_system_desc(
            FixTransformDesc,
            "fix_transform_system",
            &["wall_body_system"],
        );

    let level_config = LevelConfig::load(assets_dir.join("prefab/level.ron"))?;

    let state = GameState::new(pew_config, level_config);

    let mut game = Application::new(assets_dir, state, game_data)?;
    game.run();

    Ok(())
}
