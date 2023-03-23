use sdl2::{render::{Canvas, Texture}, video::Window};

pub enum LightSpriteEvent {
    None,
    DeleteMe,
    CreatePowerUpItem,
}

pub trait LightSprite {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
    fn draw(&self, texture: &mut Texture, canvas: &mut Canvas<Window>);
    fn on_next_frame(&mut self) -> LightSpriteEvent { LightSpriteEvent::None }
    fn get_width(&self) -> u32 { 32 }
    fn get_height(&self) -> u32 { 32 }
}