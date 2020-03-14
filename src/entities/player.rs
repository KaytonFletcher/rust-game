use amethyst::{
    core::Transform,
    prelude::{Builder, World, WorldExt},
    renderer::SpriteRender,
    window::ScreenDimensions,
};

use crate::components::Player;

pub fn init_player(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 0.);

    world
        .create_entity()
        .with(sprites[0].clone())
        .with(Player::new())
        .with(transform)
        .build();
}
