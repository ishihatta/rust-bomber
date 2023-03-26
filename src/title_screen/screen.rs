use std::path::Path;

use sdl2::{video::{Window, WindowContext}, render::{Canvas, Texture, TextureCreator, TextureQuery}, pixels::Color, rect::Rect, ttf::{Font, Sdl2TtfContext}, EventPump, keyboard::Scancode, mixer::{Chunk, Music}};
use sdl2::image::LoadTexture;

use crate::game_screen::player_type::PlayerType;
use crate::screen::{Screen, ScreenEvent};

struct MenuItem<'a> {
    text: &'a str,
    player_type_1: PlayerType,
    player_type_2: PlayerType,
}

const MENU_ITEMS: [MenuItem; 4] = [
    MenuItem { text: "HUMAN VS HUMAN", player_type_1: PlayerType::HUMAN, player_type_2: PlayerType::HUMAN },
    MenuItem { text: "HUMAN VS AI", player_type_1: PlayerType::HUMAN, player_type_2: PlayerType::AI },
    MenuItem { text: "AI VS HUMAN", player_type_1: PlayerType::AI, player_type_2: PlayerType::HUMAN },
    MenuItem { text: "AI VS AI", player_type_1: PlayerType::AI, player_type_2: PlayerType::AI },
];
const MENU_ITEM_X: i32 = 300;
const MENU_ITEM_Y_START: i32 = 260;
const MENU_ITEM_Y_STEP: i32 = 40;
const JINGLE_TIME: i32 = 190;

pub struct TitleScreen<'a> {
    // テクスチャ
    pub logo_image: Texture<'a>,
    pub cursor_image: Texture<'a>,

    // フォント
    font16: Font<'a, 'a>,

    // テクスチャ生成器
    texture_creator: &'a TextureCreator<WindowContext>,

    // カーソル位置
    cursor: usize,

    // 前フレームでのカーソルの移動
    previous_move: i32,

    // ゲーム画面に遷移しているときの状態
    going_to_game_screen_state: i32,

    // ゲーム開始ジングル
    start_game_sound: Option<Chunk>,

    // BGM
    bgm_music: Option<Music<'a>>,
}

impl TitleScreen<'_> {
    pub fn new<'a>(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> TitleScreen<'a> {
        let screen = TitleScreen {
            logo_image: texture_creator.load_texture(Path::new("res/image/logo.png")).unwrap(),
            cursor_image: texture_creator.load_texture(Path::new("res/image/pipo-charachip018b.png")).unwrap(),
            font16: ttf_context.load_font(Path::new("res/font/m12.ttf"), 16).unwrap(),
            texture_creator,
            cursor: 0,
            previous_move: 0,
            going_to_game_screen_state: -1,
            start_game_sound: Chunk::from_file(Path::new("res/sound/start_game.mp3")).ok(),
            bgm_music: Music::from_file(Path::new("res/sound/title_bgm.mp3")).ok(),
        };

        // BGMの再生
        if let Some(music) = &screen.bgm_music {
            sdl2::mixer::Music::set_volume(128);
            if let Err(error) = music.play(1) {
                println!("Failed to play BGM: {}", error);
            }
        }

        screen
    }

    fn draw_text(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, text: &str, color: Color) {
        let surface = self.font16
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let dst_rect = Some(Rect::new(x, y, width, height));
        if let Err(error) = canvas.copy(&texture, None, dst_rect) {
            println!("Failed to draw text: {}", error);
        }
    }
}

impl Screen for TitleScreen<'_> {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // ロゴの描画
        {
            let TextureQuery { width, height, .. } = self.logo_image.query();
            if let Err(error) = canvas.copy(
                &self.logo_image,
                None,
                Some(Rect::new(400 - width as i32 / 2, 120 - height as i32 / 2, width, height))
            ) {
                println!("Failed to copy texture {}", error);
            }
        }

        // 選択肢の描画
        for i in 0..MENU_ITEMS.len() {
            let y = MENU_ITEM_Y_START + MENU_ITEM_Y_STEP * i as i32;
            let color = if i == self.cursor {
                if self.going_to_game_screen_state >= 0 && self.going_to_game_screen_state % 12 < 6 {
                    Color::RGB(160, 160, 160)
                } else {
                    Color::RGB(255, 160, 160)
                }
            } else {
                Color::RGB(160, 160, 160)
            };
            self.draw_text(canvas, MENU_ITEM_X, y, MENU_ITEMS[i].text, color);
        }

        // カーソルの描画
        if let Err(error) = canvas.copy(
            &self.cursor_image,
            Some(Rect::new(0, 0, 32, 32)),
            Some(Rect::new(MENU_ITEM_X - 40, MENU_ITEM_Y_START + MENU_ITEM_Y_STEP * self.cursor as i32 - 8, 32, 32))
        ) {
            println!("Failed to copy texture {}", error);
        }

        // フェードアウト
        if self.going_to_game_screen_state > JINGLE_TIME - 15 {
            canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
            canvas.set_draw_color(Color::RGBA(0, 0, 0, (255 * (self.going_to_game_screen_state - (JINGLE_TIME - 15)) / 15) as u8));
            if let Err(error) = canvas.fill_rect(Rect::new(0, 0, 800, 480)) {
                println!("Failed to fill rect: {}", error);
            }
        }

        canvas.present();
    }

    fn on_next_frame(&mut self, event_pump: &EventPump) -> ScreenEvent {
        // ゲーム画面への遷移中の場合は何もしない
        if self.going_to_game_screen_state >= 0 {
            self.going_to_game_screen_state += 1;
            if self.going_to_game_screen_state >= JINGLE_TIME {
                let item = &MENU_ITEMS[self.cursor];
                return ScreenEvent::GoToGameScreen(item.player_type_1, item.player_type_2);
            }
            return ScreenEvent::None;
        }

        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::Up) || keyboard_state.is_scancode_pressed(Scancode::W) {
            if self.previous_move != -1 {
                self.previous_move = -1;
                if self.cursor == 0 {
                    self.cursor = MENU_ITEMS.len() - 1;
                } else {
                    self.cursor -= 1;
                }
            }
        } else if keyboard_state.is_scancode_pressed(Scancode::Down) || keyboard_state.is_scancode_pressed(Scancode::S) {
            if self.previous_move != 1 {
                self.previous_move = 1;
                if self.cursor == MENU_ITEMS.len() - 1 {
                    self.cursor = 0;
                } else {
                    self.cursor += 1;
                }
            }
        } else {
            self.previous_move = 0;

            // 決定キー
            if keyboard_state.is_scancode_pressed(Scancode::Space) || keyboard_state.is_scancode_pressed(Scancode::Num1) || keyboard_state.is_scancode_pressed(Scancode::Slash) {
                // BGM停止
                sdl2::mixer::Music::halt();
                // ジングル再生
                if let Some(chunk) = &self.start_game_sound {
                    if let Err(error) = sdl2::mixer::Channel::all().play(chunk, 0) {
                        println!("Failed to play chunk: {}", error);
                    }
                }
                // ゲーム画面への画面遷移を開始する
                self.going_to_game_screen_state = 0;
            }
        }

        ScreenEvent::None
    }
}