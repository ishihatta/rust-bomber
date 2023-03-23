#[derive(Copy, Clone)]
pub enum Movement {
    NONE, LEFT, UP, RIGHT, DOWN
}

pub struct PlayerInput {
    pub movement: Movement,
    pub fire: bool,
}

impl PlayerInput {
    pub fn new(movement: Movement, fire: bool) -> PlayerInput {
        PlayerInput {
            movement,
            fire,
        }
    }
}