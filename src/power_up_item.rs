use sdl2::{video::Window, render::{Canvas, Texture}, rect::Rect};

use crate::{light_sprite::LightSprite, light_sprite::LightSpriteEvent};

pub struct PowerUpItem {
    x: i32,
    y: i32,
    move_time: f32,
}

impl PowerUpItem {
    pub fn new(x: i32, y: i32) -> PowerUpItem {
        PowerUpItem {
            x,
            y,
            move_time: 0f32,
        }
    }
}

impl LightSprite for PowerUpItem {
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }

    fn draw(&self, texture: &mut Texture, canvas: &mut Canvas<Window>) {
        let step = (self.move_time / 0.2f32) as i32 % 3;
        let width = self.get_width();
        let height = self.get_height();
        let src_x = step * width as i32;

        canvas.copy(
            texture,
            Some(Rect::new(src_x, 0, width, height)),
            Some(Rect::new(self.x, self.y, width, height)),
        ).expect("Failure to draw canvas");
    }

    fn on_next_frame(&mut self) -> LightSpriteEvent {
        self.move_time += 1.0 / 60.0;
        LightSpriteEvent::None
    }
}