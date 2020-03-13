use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, Join, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::state::Player;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {

        for (player, transform) in (&players, &mut transforms).join() {
           
        
            let vert_movement = input.axis_value("player_updown");

            let horiz_movement = input.axis_value("player_leftright");
            if let Some(mv_amount) = vert_movement {
                if mv_amount != 0.0 {
                    println!("Player moving vertically {}", mv_amount);
                    let scaled_amount = player.speed * mv_amount as f32;  
                    transform.prepend_translation_y(scaled_amount);
                                
                }
            }
            if let Some(mv_amount) = horiz_movement {
                if mv_amount != 0.0 {
                    println!("Player moving horizontally {}", mv_amount);

                    let scaled_amount = player.speed * mv_amount as f32;  
                    transform.prepend_translation_x(scaled_amount);
                }
            }
        }
    }
}
