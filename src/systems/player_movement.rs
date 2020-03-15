use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Direction, Player, PlayerState};

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut players, input): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {
            let vert_movement = input.axis_value("player_updown");
            let horiz_movement = input.axis_value("player_leftright");

            let mut move_horiz = 0.0;
            let mut move_vert = 0.0;

            if let Some(mv_amount) = vert_movement {
                if mv_amount != 0.0 {
                    move_vert = mv_amount;
                }

                if let Some(mv_horiz_amount) = horiz_movement {
                    if mv_horiz_amount != 0.0 {
                        move_horiz = mv_horiz_amount;
                    }
                }
            } else if let Some(mv_horiz_amount) = horiz_movement {
                if mv_horiz_amount != 0.0 {
                    move_horiz = mv_horiz_amount;
                }
            }

            if move_vert != 0.0 && move_horiz != 0.0 {
                let speed = (player.speed.powf(2.0) / 2.0).sqrt();
                let scaled_amount_y = speed * move_vert;
                let scaled_amount_x = speed * move_horiz;

                transform.prepend_translation_y(scaled_amount_y);
                transform.prepend_translation_x(scaled_amount_x);
                if move_horiz > 0.0 {
                    // println!("Player moving right diagonally {}", speed);
                    player.state = PlayerState::Running(Direction::Right);
                } else {
                    // println!("Player moving left diagonally {}", speed);
                    player.state = PlayerState::Running(Direction::Left);
                }
            } else if move_vert != 0.0 {
                let scaled_amount = player.speed * move_vert;
                transform.prepend_translation_y(scaled_amount);

                if move_vert > 0.0 {
                    // println!("Player moving up {}", move_vert);
                    player.state = PlayerState::Running(Direction::Up);
                } else {
                    // println!("Player moving down {}", move_vert);
                    player.state = PlayerState::Running(Direction::Down);
                }
            } else if move_horiz != 0.0 {
                let scaled_amount = player.speed * move_horiz;
                transform.prepend_translation_x(scaled_amount);

                if move_horiz > 0.0 {
                    // println!("Player moving right {}", move_horiz);
                    player.state = PlayerState::Running(Direction::Right);
                } else {
                    // println!("Player moving left {}", move_horiz);
                    player.state = PlayerState::Running(Direction::Left);
                }
            } else {
                // println!("PLAYER IS IDLING");
                player.state = PlayerState::Idling;
            }
        }
    }
}
