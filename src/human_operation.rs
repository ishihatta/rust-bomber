use sdl2::keyboard::{Scancode, KeyboardState};

use crate::ai::ai_player::AIPlayerAdditionalInfo;
use crate::player_operation::PlayerOperation;
use crate::player_input::{PlayerInput, Movement};

struct KeyAssignment {
    left: Scancode,
    up: Scancode,
    right: Scancode,
    down: Scancode,
    fire: Scancode,
}

const KEY_ASSIGNMENTS: [KeyAssignment; 2] = [
    KeyAssignment { left: Scancode::A, up: Scancode::W, right: Scancode::D, down: Scancode::S, fire: Scancode::Num1 },
    KeyAssignment { left: Scancode::Left, up: Scancode::Up, right: Scancode::Right, down: Scancode::Down, fire: Scancode::Slash },
];

pub struct HumanOperation {
    pub player_number: usize,
}

impl PlayerOperation for HumanOperation {
    fn get_player_input(&mut self, keyboard_state: &KeyboardState, _: Option<AIPlayerAdditionalInfo>) -> PlayerInput {
        let movement = if keyboard_state.is_scancode_pressed(KEY_ASSIGNMENTS[self.player_number].left) {
            Movement::LEFT
        } else if keyboard_state.is_scancode_pressed(KEY_ASSIGNMENTS[self.player_number].up) {
            Movement::UP
        } else if keyboard_state.is_scancode_pressed(KEY_ASSIGNMENTS[self.player_number].right) {
            Movement::RIGHT
        } else if keyboard_state.is_scancode_pressed(KEY_ASSIGNMENTS[self.player_number].down) {
            Movement::DOWN
        } else {
            Movement::NONE
        };
        let fire = keyboard_state.is_scancode_pressed(KEY_ASSIGNMENTS[self.player_number].fire);
        PlayerInput { movement, fire }
    }
}