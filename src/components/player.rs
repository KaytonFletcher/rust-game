use amethyst::ecs::{Component, DenseVecStorage};

const PLAYER_HEIGHT: f32 = 40.0;
const PLAYER_WIDTH: f32 = 40.0;
const PLAYER_SPEED: f32 = 4.0;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PlayerState {
    Idling,
    Running,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::default(),
            speed: PLAYER_SPEED,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
        }
    }
}

