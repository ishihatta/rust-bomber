use sdl2::keyboard::KeyboardState;

use crate::{player_input::PlayerInput, ai::ai_player::AIPlayerAdditionalInfo};

pub trait PlayerOperation {
    fn get_player_input(&mut self, keyboard_state: &KeyboardState, ai_additional_info: Option<AIPlayerAdditionalInfo>) -> PlayerInput;
}