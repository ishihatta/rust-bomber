use sdl2::keyboard::KeyboardState;

use super::player_input::PlayerInput;
use crate::ai::ai_player::AIPlayerAdditionalInfo;

pub trait PlayerOperation {
    fn get_player_input(&mut self, keyboard_state: &KeyboardState, ai_additional_info: Option<AIPlayerAdditionalInfo>) -> PlayerInput;
}