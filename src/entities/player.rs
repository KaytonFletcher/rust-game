use amethyst::{
    assets::{Handle, Prefab},
    core::Transform,
    prelude::{Builder, World, WorldExt},
    window::ScreenDimensions,
};

use crate::components::{animation, Player};

pub fn init_player(
    world: &mut World,
    prefab: Handle<Prefab<animation::AnimationPrefabData>>,
    dimensions: &ScreenDimensions,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 0.);

    world
        .create_entity()
        .with(animation::Animation::new(
            animation::AnimationId::Idle,
            vec![animation::AnimationId::Idle, animation::AnimationId::MoveLeft,
            animation::AnimationId::MoveRight, animation::AnimationId::MoveUp, animation::AnimationId::MoveDown],
        ))
        .with(prefab)
        .with(Player::new())
        .with(transform)
        .build();
}
