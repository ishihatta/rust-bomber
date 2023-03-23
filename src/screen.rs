use sdl2::{EventPump, video::Window, render::Canvas};

use crate::game_screen::player_type::PlayerType;

pub enum ScreenEvent {
    None,
    GoToGameScreen(PlayerType, PlayerType),
    ReturnToTitleScreen,
}

pub trait Screen {
    fn draw(&mut self, canvas: &mut Canvas<Window>);
    fn on_next_frame(&mut self, event_pump: &mut EventPump) -> ScreenEvent;
}