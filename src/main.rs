use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        sprite::SpriteRender
    },
    utils::application_root_dir,
};

mod components;
mod entities;
mod resources;
mod states;
mod systems;

use components::animation;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets = app_root.join("assets");
    let display_config = assets.join("display_config.ron");
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_system_desc(
            PrefabLoaderSystemDesc::<animation::AnimationPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<animation::AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(
            systems::animations::PlayerAnimationSystem,
            "player_animation_system",
            &[],
        )
        .with(systems::animations::AnimationControlSystem, "animation_control_system", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(assets, states::LoadState::default(), game_data)?;
    game.run();

    Ok(())
}
