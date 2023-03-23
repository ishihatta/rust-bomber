use sdl2::{render::{Canvas, Texture}, video::Window, rect::Rect};

use crate::{light_sprite::LightSprite, light_sprite::LightSpriteEvent};

#[derive(Copy, Clone)]
pub enum Position {
    CENTER = 0,
    VERTICAL,
    HORIZONTAL,
    LEFT,
    TOP,
    RIGHT,
    BOTTOM,
}

pub struct Explosion {
    x: i32,
    y: i32,
    position: Position,
    remain_time: i32,
}

impl Explosion {
    pub fn new(x: i32, y: i32, position: Position) -> Explosion {
        Explosion {
            x,
            y,
            position,
            remain_time: 30,
        }
    }
}

impl LightSprite for Explosion {
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }

    fn draw(&self, texture: &mut Texture, canvas: &mut Canvas<Window>) {
        let width = self.get_width();
        let height = self.get_height();
        let src_x = if self.remain_time < 3 || self.remain_time > 27 {
            32
        } else {
            0
        };
        let src_y = (self.position as i32) * height as i32;

        canvas.copy(
            texture,
            Some(Rect::new(src_x, src_y, width, height)),
            Some(Rect::new(self.x, self.y, width, height)),
        ).expect("Failure to draw canvas");
    }

    fn on_next_frame(&mut self) -> LightSpriteEvent {
        self.remain_time -= 1;
        if self.remain_time <= 0 {
            LightSpriteEvent::DeleteMe
        } else {
            LightSpriteEvent::None
        }
    }
}