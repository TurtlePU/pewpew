mod components;
mod systems;
mod input_bindings;
mod pewpew;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::InputBundle
};
use crate::{
    input_bindings::InputBindings,
    pewpew::MyState
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let input_bundle = InputBundle::<InputBindings>::new()
        .with_bindings_from_file(config_dir.join("bindings.ron"))?;

    let render_to_window = RenderToWindow::from_config_path(display_config_path)?
        .with_clear([0.34, 0.36, 0.52, 1.0]);

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(render_to_window)
        .with_plugin(RenderFlat2D::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(rendering_bundle)?
        .with_bundle(TransformBundle::new())?
        .with(systems::ControlSystem, "control_system", &["input_system"]);

    let mut game = Application::new(assets_dir, MyState, game_data)?;
    game.run();

    Ok(())
}
