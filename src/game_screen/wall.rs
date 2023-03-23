extern crate rand;

use sdl2::{video::Window, render::{Canvas, Texture}, rect::Rect};
use rand::Rng;

use super::{light_sprite::LightSprite, light_sprite::LightSpriteEvent};

const TIME_TO_MELT: i32 = 30;

pub struct Wall {
    x: i32,
    y: i32,
    pub is_breakable: bool,
    melt_state: i32,
}

impl Wall {
    pub fn new(x: i32, y: i32, is_breakable: bool) -> Wall {
        Wall {
            x,
            y,
            is_breakable,
            melt_state: 0,
        }
    }

    pub fn start_melting(&mut self) {
        if self.melt_state == 0 {
            self.melt_state = 1;
        }
    }

    pub fn is_melting(&self) -> bool {
        self.melt_state > 0
    }
}

impl LightSprite for Wall {
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }

    fn draw(&self, texture: &mut Texture, canvas: &mut Canvas<Window>) {
        if self.melt_state > 0 {
            texture.set_color_mod(255, 0, 0);
            texture.set_alpha_mod(((TIME_TO_MELT - self.melt_state) as f32 / TIME_TO_MELT as f32 * 255f32) as u8);
            texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        } else {
            texture.set_color_mod(255, 255, 255);
            texture.set_alpha_mod(255);
            texture.set_blend_mode(sdl2::render::BlendMode::None);
        }
        canvas.copy(
            texture,
            None,
            Some(Rect::new(self.x, self.y, self.get_width(), self.get_height())),
        ).expect("Failure to draw canvas");
    }

    fn on_next_frame(&mut self) -> LightSpriteEvent {
        if self.melt_state > 0 {
            self.melt_state += 1;
            if self.melt_state >= TIME_TO_MELT {
                // 一定の確率でパワーアップアイテムが出る
                let mut rng = rand::thread_rng();
                if rng.gen_range(0, 100) < 10 {
                    return LightSpriteEvent::CreatePowerUpItem
                }
                return LightSpriteEvent::DeleteMe
            }
        }
        LightSpriteEvent::None
    }
}