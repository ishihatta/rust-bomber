use sdl2::{render::{Canvas, Texture}, video::Window, rect::Rect};

use super::{light_sprite::LightSprite, light_sprite::LightSpriteEvent, constants};

#[derive(Clone)]
pub struct Bomb {
    x: i32,
    y: i32,
    pub power: i32,
    move_time: f32,
    pub remain_time: i32,
}

impl Bomb {
    pub fn new(x: i32, y: i32, power: i32) -> Bomb {
        Bomb {
            x,
            y,
            power,
            move_time: 0f32,
            remain_time: constants::BOMB_TIME,
        }
    }
}

impl LightSprite for Bomb {
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }

    fn draw(&self, texture: &mut Texture, canvas: &mut Canvas<Window>) {
        let step = (self.move_time / 0.2f32) as i32 % 3;
        let width = self.get_width();
        let height = self.get_height();
        canvas.copy(
            texture,
            Some(Rect::new(width as i32 * step, 0, width, height)),
            Some(Rect::new(self.x, self.y, width, height)),
        ).expect("Failure to draw canvas");
    }

    fn on_next_frame(&mut self) -> LightSpriteEvent {
        self.move_time += 1.0 / 60.0;

        // タイムアウトしたら爆発する
        self.remain_time -= 1;
        if self.remain_time <= 0 {
            LightSpriteEvent::DeleteMe
        } else {
            LightSpriteEvent::None
        }
    }
}