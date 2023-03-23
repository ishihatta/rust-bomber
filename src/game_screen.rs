extern crate sdl2;
extern crate rand;

use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::ttf::{Sdl2TtfContext, Font};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Scancode;
use sdl2::mixer::{Chunk, Channel, Music};
use rand::Rng;
use std::path::Path;

use crate::bomb::Bomb;
use crate::explosion;
use crate::explosion::Explosion;
use crate::light_sprite::{LightSprite, LightSpriteEvent};
use crate::player::Player;
use crate::player_type::PlayerType;
use crate::wall::Wall;
use crate::power_up_item::PowerUpItem;
use crate::constants;

pub const MAP_WIDTH: i32 = 25;
pub const MAP_HEIGHT: i32 = 15;

enum State {
    Playing,
    Player1Won,
    Player2Won,
    DrawGame,
}

enum Alignment {
    Left, Center, Right
}

pub struct GameScreen<'a> {
    state: State,

    // テクスチャ
    pub wall_image: Texture<'a>,
    pub breakable_wall_image: Texture<'a>,
    pub bomb_image: Texture<'a>,
    pub player1_image: Texture<'a>,
    pub player2_image: Texture<'a>,
    pub explosion_image: Texture<'a>,
    pub power_up_item_image: Texture<'a>,

    // 効果音
    pub explosion_sound: Option<Chunk>,
    pub set_bomb_sound: Option<Chunk>,
    pub walk_sound: Option<Chunk>,
    pub power_up_sound: Option<Chunk>,
    pub crash_sound: Option<Chunk>,

    // BGM
    pub bgm_music: Option<Music<'a>>,

    // スプライトの配列
    pub players: Vec<Player>,
    pub walls: Vec<Wall>,
    pub bombs: Vec<Bomb>,
    pub explosions: Vec<Explosion>,
    pub power_up_items: Vec<PowerUpItem>,

    // フォント
    font16: Font<'a, 'a>,
    font32: Font<'a, 'a>,

    // テクスチャ生成器
    texture_creator: &'a TextureCreator<WindowContext>,
}

impl GameScreen<'_> {
    pub fn new<'a>(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> GameScreen<'a> {
        GameScreen {
            state: State::Playing,
            wall_image: texture_creator.load_texture(Path::new("res/image/wall.png")).unwrap(),
            breakable_wall_image: texture_creator.load_texture(Path::new("res/image/breakable_wall.png")).unwrap(),
            bomb_image: texture_creator.load_texture(Path::new("res/image/pipo-simpleenemy01b.png")).unwrap(),
            player1_image: texture_creator.load_texture(Path::new("res/image/pipo-charachip018b.png")).unwrap(),
            player2_image: texture_creator.load_texture(Path::new("res/image/pipo-charachip018a.png")).unwrap(),
            explosion_image: texture_creator.load_texture(Path::new("res/image/explosion.png")).unwrap(),
            power_up_item_image: texture_creator.load_texture(Path::new("res/image/pipo-etcchara003.png")).unwrap(),
            explosion_sound: Chunk::from_file(Path::new("res/sound/explosion.mp3")).ok(),
            set_bomb_sound: Chunk::from_file(Path::new("res/sound/set_bomb.mp3")).ok(),
            walk_sound: Chunk::from_file(Path::new("res/sound/walk.mp3")).ok(),
            power_up_sound: Chunk::from_file(Path::new("res/sound/power_up.mp3")).ok(),
            crash_sound: Chunk::from_file(Path::new("res/sound/crash.mp3")).ok(),
            bgm_music: Music::from_file(Path::new("res/sound/Daily_News.mp3")).ok(),
            players: Vec::new(),
            walls: Vec::new(),
            bombs: Vec::new(),
            explosions: Vec::new(),
            power_up_items: Vec::new(),
            font16: ttf_context.load_font(Path::new("res/font/m12.ttf"), 16).unwrap(),
            font32: ttf_context.load_font(Path::new("res/font/m12.ttf"), 32).unwrap(),
            texture_creator,
        }
    }

    pub fn start_game(&mut self) {
        self.state = State::Playing;

        // Playerの生成
        self.players.clear();
        self.players.push(Player::new(
                0,
                PlayerType::AI,
                constants::CHARACTER_SIZE,
                constants::CHARACTER_SIZE,
        ));
        self.players.push(Player::new(
                1,
                PlayerType::AI,
                constants::SCREEN_WIDTH - constants::CHARACTER_SIZE * 2,
                constants::SCREEN_HEIGHT - constants::CHARACTER_SIZE * 2,
        ));

        // 外壁の生成
        self.walls.clear();
        for x in 0..MAP_WIDTH {
            let xf = x * constants::CHARACTER_SIZE;
            self.walls.push(Wall::new(xf, 0, false));
            self.walls.push(Wall::new(xf, 14 * constants::CHARACTER_SIZE, false));
        }
        for y in 1..(MAP_HEIGHT - 1) {
            let yf = y * constants::CHARACTER_SIZE;
            self.walls.push(Wall::new(0, yf, false));
            self.walls.push(Wall::new(24 * constants::CHARACTER_SIZE, yf, false));
        }

        // 壁の生成
        for y in 1..(MAP_HEIGHT - 1) {
            let yf = y * constants::CHARACTER_SIZE;
            for x in 1..(MAP_WIDTH - 1) {
                let xf = x * constants::CHARACTER_SIZE;
                if x % 2 == 0 && y % 2 == 0 {
                    // 壊せない壁
                    self.walls.push(Wall::new(xf, yf, false));
                } else {
                    // 壊せる壁
                    if x < 3 && y < 3 || x > MAP_WIDTH - 4 && y > MAP_HEIGHT - 4 {
                        // プレイヤー出現位置の近くには壁は作らない
                    } else {
                        let mut rng = rand::thread_rng();
                        if rng.gen_range(0, 100) < 50 {
                            self.walls.push(Wall::new(xf, yf, true));
                        }
                    }
                }
            }
        }

        // その他のオブジェクトの初期化
        self.bombs.clear();
        self.explosions.clear();
        self.power_up_items.clear();

        // 効果音の初期化
        sdl2::mixer::Channel::all().halt();

        // BGMの再生
        if let Some(music) = &self.bgm_music {
            sdl2::mixer::Music::set_volume(90);
            if let Err(error) = music.play(-1) {
                println!("Failure to play BGM: {}", error);
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 178, 0));
        canvas.clear();

        // 各種オブジェクトの描画
        for sprite in &self.walls { sprite.draw( if sprite.is_breakable { &mut self.breakable_wall_image } else { &mut self.wall_image } , canvas); }
        for sprite in &self.bombs { sprite.draw(&mut self.bomb_image, canvas); }
        for sprite in &self.power_up_items { sprite.draw(&mut self.power_up_item_image, canvas); }
        for sprite in &self.explosions { sprite.draw(&mut self.explosion_image, canvas); }
        for sprite in &self.players { sprite.draw(if sprite.player_number == 0 { &mut self.player1_image } else { &mut self.player2_image }, canvas); }
        // ゲーム終了時の描画
        match self.state {
            State::Playing => (),
            State::Player1Won => {
                self.draw_text(canvas, Rect::new(0, 0, 800, 480), "PLAYER 1 WIN", Color::RGB(178, 0, 0), &self.font32, Alignment::Center);
            }
            State::Player2Won => {
                self.draw_text(canvas, Rect::new(0, 0, 800, 480), "PLAYER 2 WIN", Color::RGB(0, 0, 255), &self.font32, Alignment::Center);
            }
            State::DrawGame => {
                self.draw_text(canvas, Rect::new(0, 0, 800, 480), "DRAW GAME", Color::RGB(255, 255, 255), &self.font32, Alignment::Center);
            }
        }
        // 画面上部に表示する各プレイヤーの状態描画
        self.draw_text(canvas, Rect::new(0, 0, 800, 16), &format!("PLAYER 1 POWER {}", self.players[0].power), Color::RGB(178, 0, 0), &self.font16, Alignment::Left);
        self.draw_text(canvas, Rect::new(800 - 256, 0, 800, 16), &format!("PLAYER 2 POWER {}", self.players[1].power), Color::RGB(0, 0, 255), &self.font16, Alignment::Left);

        canvas.present();
    }

    pub fn on_next_frame(&mut self, event_pump: &EventPump) {
        // キーボード状態取得
        let keyboard_state = event_pump.keyboard_state();

        // プレイヤーの移動処理
        for i in 0..self.players.len() {
            self.players[i].push_position();
            Player::move_for_next_frame(self, i, &keyboard_state);
        }

        // プレイヤー同士の衝突回避
        self.players_collision_detect();

        // プレイヤー、パワーアップアイテム、壁、爆発の状態変化
        let mut new_power_up_items: Vec<PowerUpItem> = Vec::new();
        Self::sprites_state_transition(&mut self.players, &mut new_power_up_items);
        Self::sprites_state_transition(&mut self.power_up_items, &mut new_power_up_items);
        Self::sprites_state_transition(&mut self.walls, &mut new_power_up_items);
        Self::sprites_state_transition(&mut self.explosions, &mut new_power_up_items);

        // パワーアップアイテムの追加
        for item in new_power_up_items {
            self.power_up_items.push(item);
        }

        // 移動後の処理
        for i in 0..self.players.len() {
            Player::after_next_frame(self, i);
        }

        // 爆弾の状態変化
        let mut new_explode_bomb: Vec<Bomb> = Vec::new();
        self.bombs.retain_mut(|bomb|
            if let LightSpriteEvent::DeleteMe = bomb.on_next_frame() {
                // 爆発した爆弾をリストに入れておく
                new_explode_bomb.push(bomb.clone());
                false
            } else {
                true
            }
        );

        // 爆発の生成
        if !new_explode_bomb.is_empty() {
            Self::play_chunk(&self.explosion_sound, false);
            for bomb in new_explode_bomb.iter() {
                self.explosions.push(Explosion::new(bomb.get_x(), bomb.get_y(), explosion::Position::CENTER));
                self.expand_explosion(bomb, -1, 0);
                self.expand_explosion(bomb, 1, 0);
                self.expand_explosion(bomb, 0, -1);
                self.expand_explosion(bomb, 0, 1);
            }
        }

        // ゲーム状態の変化
        if let State::Playing = self.state {
            if self.players[0].is_dead() && self.players[1].is_dead() {
                self.state = State::DrawGame;
            } else if self.players[0].is_dead() {
                self.state = State::Player2Won;
            } else if self.players[1].is_dead() {
                self.state = State::Player1Won;
            }
        } else {
            // ゲームが終わっている状態でスペースキーが押されると最初からになる
            if keyboard_state.is_scancode_pressed(Scancode::Space) {
                self.start_game();
            }
        }

        // メインメニューに戻る
        if keyboard_state.is_scancode_pressed(Scancode::Escape) {
        //     game.returnToMainMenu()
        }
    }

    pub fn play_chunk(chunk: &Option<Chunk>, is_loop: bool) -> Option<Channel> {
        if let Some(sound) = chunk {
            match sdl2::mixer::Channel::all().play(sound, if is_loop { -1 } else { 1 }) {
                Ok(channel) => Some(channel),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    fn players_collision_detect(&mut self) {
        if self.players[0].is_dead() || self.players[1].is_dead() {
            return;
        }
        if (self.players[0].get_x() - self.players[1].get_x()).abs() < constants::CHARACTER_SIZE &&
            (self.players[0].get_y() - self.players[1].get_y()).abs() < constants::CHARACTER_SIZE {
            let player0_is_not_cancelable = (self.players[0].pushed_x - self.players[1].get_x()).abs() < constants::CHARACTER_SIZE &&
                (self.players[0].pushed_y - self.players[1].get_y()).abs() < constants::CHARACTER_SIZE;
            let player1_is_not_cancelable = (self.players[0].get_x() - self.players[1].pushed_x).abs() < constants::CHARACTER_SIZE &&
                (self.players[0].get_y() - self.players[1].pushed_y).abs() < constants::CHARACTER_SIZE;
            if !player0_is_not_cancelable && player1_is_not_cancelable {
                self.players[0].pop_position();
            } else if player0_is_not_cancelable && !player1_is_not_cancelable {
                self.players[1].pop_position();
            } else {
                self.players[0].pop_position();
                self.players[1].pop_position();
            }
        }
    }

    fn expand_explosion(&mut self, bomb: &Bomb, xx: i32, yy: i32) {
        for n in 1..(bomb.power + 1) {
            let px = bomb.get_x() + xx * n * constants::CHARACTER_SIZE;
            let py = bomb.get_y() + yy * n * constants::CHARACTER_SIZE;

            // 壁があるか？
            if let Some(wall) = self.walls.iter_mut().find(|w| w.get_x() == px && w.get_y() == py) {
                // 壁の破壊
                if wall.is_breakable {
                    wall.start_melting();
                }
                return;
            }
            // 爆弾があったら誘爆する
            if let Some(b) = self.bombs.iter_mut().find(|b| b.get_x() == px && b.get_y() == py) {
                b.remain_time = 1;
                return;
            }
            // パワーアップアイテムがあったら破壊する
            for i in 0..self.power_up_items.len() {
                let item = self.power_up_items.get(i).unwrap();
                if item.get_x() == px && item.get_y() == py {
                    self.power_up_items.remove(i);
                    return;
                }
            }
            // 新しい爆発を生成する
            let position: explosion::Position = if xx == 0 {
                if n == bomb.power {
                    if yy > 0 { explosion::Position::BOTTOM } else { explosion::Position::TOP }
                } else {
                    explosion::Position::VERTICAL
                }
            } else {
                if n == bomb.power {
                    if xx > 0 { explosion::Position::RIGHT } else { explosion::Position::LEFT }
                } else {
                    explosion::Position::HORIZONTAL
                }
            };
            self.explosions.push(Explosion::new(px, py, position));
        }
    }

    fn sprites_state_transition<T: LightSprite>(sprites: &mut Vec<T>, new_power_up_items: &mut Vec<PowerUpItem>) {
        sprites.retain_mut(|s|
            match s.on_next_frame() {
                LightSpriteEvent::DeleteMe => false,
                LightSpriteEvent::CreatePowerUpItem => {
                    new_power_up_items.push(PowerUpItem::new(s.get_x(), s.get_y()));
                    false
                }
                _ => true
            }
        );
    }

    fn draw_text(&self, canvas: &mut Canvas<Window>, target: Rect, text: &str, color: Color, font: &Font, alignment: Alignment) {
        let surface = font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let dst_rect = match alignment {
            Alignment::Left   => Some(Rect::new(target.x, target.y, width, height)),
            Alignment::Center => Some(Rect::new(target.x + (target.w - width as i32) / 2, target.y + (target.h - height as i32) / 2, width, height)),
            Alignment::Right  => Some(Rect::new(target.right() - width as i32, target.y, width, height)),
        };
        if let Err(error) = canvas.copy(&texture, None, dst_rect) {
            println!("Failure to draw text: {}", error);
        }
    }
}