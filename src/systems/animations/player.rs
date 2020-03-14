use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{animation, Player, PlayerState};

#[derive(Default)]
pub struct PlayerAnimationSystem;

impl<'s> System<'s> for PlayerAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, animation::Animation>,
        WriteStorage<'s, AnimationControlSet<animation::AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, marines, mut animations, mut animation_control_sets) = data;

        for (entity, marine, mut animation, animation_control_set) in (
            &entities,
            &marines,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let new_animation_id = match marine.state {
                PlayerState::Running => animation::AnimationId::Move,
                _ => animation::AnimationId::Idle,
            };

            // If the new AnimationId is different to the current one, abort the
            // current animation and start the new one
            if animation.current != new_animation_id {
                println!(
                    "Updating animation for entity: {:?} from={:?}, to={:?}",
                    entity,
                    animation.current,
                    new_animation_id
                );

                animation_control_set.abort(animation.current);
                animation_control_set.start(new_animation_id);

                animation.current = new_animation_id;
            }
            // } else if new_animation_id == anAnimationId::Die {
            //     animation.show = false;
            // }
        }
    }
}
