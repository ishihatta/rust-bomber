use sdl2::keyboard::KeyboardState;
use sdl2::mixer::Channel;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::ai::ai_player::AIPlayerAdditionalInfo;
use super::light_sprite::{LightSprite, LightSpriteEvent};
use super::screen::GameScreen;
use super::player_input::PlayerInput;
use super::player_operation::PlayerOperation;
use super::player_type::PlayerType;
use super::wall::Wall;
use super::player_input::Movement;
use super::bomb::Bomb;

#[derive(Copy, Clone)]
pub enum Direction {
    DOWN = 0,
    LEFT,
    RIGHT,
    UP,
}

pub struct Player {
    pub player_number: usize,
    x: i32,
    y: i32,
    player_type: PlayerType,
    player_operation: Box<dyn PlayerOperation>,
    pub pushed_x: i32,
    pub pushed_y: i32,
    direction: Direction,
    move_time: f32,
    pub power: i32,
    death_state: i32,
    player_input: PlayerInput,
    walk_sound_channel: Option<Channel>,
}

impl Player {
    pub fn new(player_number: usize, player_type: PlayerType, x: i32, y: i32) -> Player {
        Player {
            player_number,
            x,
            y,
            player_type,
            player_operation: player_type.get_player_operation(player_number),
            pushed_x: 0,
            pushed_y: 0,
            direction: Direction::DOWN,
            move_time: 0f32,
            power: 1,
            death_state: 0,
            player_input: PlayerInput::new(Movement::NONE, false),
            walk_sound_channel: None,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.death_state > 0
    }

    pub fn move_for_next_frame(game_screen: &mut GameScreen, player_number: usize, keyboard_state: &KeyboardState) {
        // PlayerOperation に渡す追加情報を生成する
        let operation_info: Option<AIPlayerAdditionalInfo>;
        {
            let player = &game_screen.players[player_number];
            operation_info = player.player_type.get_ai_additional_info(game_screen, player_number);
        }

        let player = game_screen.players.get_mut(player_number).unwrap();
        
        if player.is_dead() {
            return;
        }

        // 移動前の位置を保存しておく
        let old_x = player.x;
        let old_y = player.y;

        // 移動
        player.player_input = player.player_operation.get_player_input(keyboard_state, operation_info);
        match player.player_input.movement {
            Movement::LEFT => {
                player.direction = Direction::LEFT;
                player.x -= 2;
            }
            Movement::RIGHT => {
                player.direction = Direction::RIGHT;
                player.x += 2;
            }
            Movement::UP => {
                player.direction = Direction::UP;
                player.y -= 2;
            }
            Movement::DOWN => {
                player.direction = Direction::DOWN;
                player.y += 2;
            }
            _ => ()
        }

        // 壁との当たり判定
        let detect_walls = game_screen.walls.iter().filter(|wall|
            (wall.get_x() - player.x).abs() < 32 && (wall.get_y() - player.y).abs() < 32
        ).collect::<Vec<&Wall>>();
        if !detect_walls.is_empty() {
            player.x = old_x;
            player.y = old_y;
        }
        if detect_walls.len() == 1 {
            let wall = detect_walls.first().unwrap();
            match player.player_input.movement {
                Movement::LEFT | Movement::RIGHT => {
                    if player.y < wall.get_y() {
                        player.y -= 2;
                    }
                    if player.y > wall.get_y() {
                        player.y += 2;
                    }
                }
                Movement::DOWN | Movement::UP => {
                    if player.x < wall.get_x() {
                        player.x -= 2;
                    }
                    if player.x > wall.get_x() {
                        player.x += 2;
                    }
                }
                _ => ()
            }
        }

        // 爆弾との当たり判定
        // 32で割り切れる場所からそうでない場所に移動しようとした場合は、移動先に爆弾があったら動かさない
        if player.x != old_x && old_x % 32 == 0 {
            let bx = if player.x > old_x { old_x + 32 } else { old_x - 32 };
            if game_screen.bombs.iter().any(|bomb| bomb.get_x() == bx && bomb.get_y() == player.y) { player.x = old_x }
        } else if player.y != old_y && old_y % 32 == 0 {
            let by = if player.y > old_y { old_y + 32 } else { old_y - 32 };
            if game_screen.bombs.iter().any(|bomb| bomb.get_y() == by && bomb.get_x() == player.x) { player.y = old_y }
        }
        // 32で割り切れない場所から移動しようとした場合は、一番近いマス以外のマスに移動しようとしている場合、移動先に爆弾があったら動かさない
        else if player.x != old_x {
            let mut bx: Option<i32> = None;
            if old_x % 32 < 32 / 2 {
                if player.x > old_x {
                    bx = Some((old_x / 32 + 1) * 32);
                }
            } else {
                if player.x < old_x {
                    bx = Some((old_x / 32) * 32);
                }
            }
            if let Some(bx) = bx {
                if game_screen.bombs.iter().any(|bomb| bomb.get_x() == bx && bomb.get_y() == player.y) { player.x = old_x }
            }
        } else if player.y != old_y {
            let mut by: Option<i32> = None;
            if old_y % 32 < 32 / 2 {
                if player.y > old_y {
                    by = Some((old_y / 32 + 1) * 32);
                }
            } else {
                if player.y < old_y {
                    by = Some((old_y / 32) * 32);
                }
            }
            if let Some(by) = by {
                if game_screen.bombs.iter().any(|bomb| bomb.get_x() == player.x && bomb.get_y() == by) { player.y = old_y }
            }
        }

        // 実際に移動させる
        if player.x != old_x || player.y != old_y {
            player.move_time += 1.0 / 60.0;
            if let None = player.walk_sound_channel {
                player.walk_sound_channel = GameScreen::play_chunk(&game_screen.walk_sound, true);
            }
        } else {
            if let Some(channel) = player.walk_sound_channel {
                channel.halt();
                player.walk_sound_channel = None;
            }
        }
    }

    pub fn after_next_frame(game_screen: &mut GameScreen, player_number: usize) {
        let player = game_screen.players.get_mut(player_number).unwrap();

        if player.is_dead() {
            return;
        }

        // パワーアップアイテムとの当たり判定
        game_screen.power_up_items.retain(|item|
            if (item.get_x() - player.x).abs() < 32 && (item.get_y() - player.y).abs() < 32 {
                player.power += 1;
                GameScreen::play_chunk(&game_screen.power_up_sound, false);
                false
            } else {
                true
            }
        );

        // 爆弾の設置
        if player.player_input.fire {
            let bx = (player.x + 32 / 2) / 32 * 32;
            let by = (player.y + 32 / 2) / 32 * 32;
            if !game_screen.bombs.iter().any(|bomb|
                bomb.get_x() == bx && bomb.get_y() == by
            ) {
                game_screen.bombs.push(Bomb::new(bx, by, player.power));
                GameScreen::play_chunk(&game_screen.set_bomb_sound, false);
            }
        }

        // 爆発との当たり判定
        if game_screen.explosions.iter().any(|explosion|
            (explosion.get_x() - player.x).abs() < 28 && (explosion.get_y() - player.y).abs() < 28
        ) {
            player.death_state = 1;
            if let Some(_) = &game_screen.bgm_music {
                sdl2::mixer::Music::halt();
            }
            GameScreen::play_chunk(&game_screen.crash_sound, false);
            if let Some(channel) = player.walk_sound_channel {
                channel.halt();
                player.walk_sound_channel = None;
            }
        }
    }

    pub fn push_position(&mut self) {
        self.pushed_x = self.x;
        self.pushed_y = self.y;
    }

    pub fn pop_position(&mut self) {
        self.x = self.pushed_x;
        self.y = self.pushed_y;
    }
}

impl LightSprite for Player {
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }

    fn draw(&self, texture: &mut Texture, canvas: &mut Canvas<Window>) {
        if self.death_state as i32 >= 60 {
            return;
        }

        // let player_image = if self.player_number == 0 { &mut game_screen.player1_image } else { &mut game_screen.player2_image };
        let width = self.get_width();
        let height = self.get_height();
        let step = (self.move_time / 0.2f32) as i32 % 3;
        let source = Some(Rect::new(
            step * width as i32,
            (self.direction as i32) * height as i32,
            width,
            height,
        ));
        if self.is_dead() {
            // 死に途中
            // let color = Color { r: 1f32, g: 0f32, b: 0f32, a: 1f32 - self.death_state as f32 / 60f32};
            texture.set_color_mod(255, 0, 0);
            texture.set_alpha_mod(((1.0 - self.death_state as f32 / 60.0) * 255.0) as u8);
            canvas.copy(
                texture,
                source,
                Some(Rect::new(
                    self.x - self.death_state, self.y - self.death_state,
                    width + self.death_state as u32 * 2, height + self.death_state as u32 * 2)
                ),
            ).expect("Failure to draw canvas");
        } else {
            texture.set_color_mod(255, 255, 255);
            texture.set_alpha_mod(255);
            canvas.copy(
                texture,
                source,
                Some(Rect::new(self.x, self.y, width, height)),
            ).expect("Failure to draw canvas");
        }
    }

    fn on_next_frame(&mut self) -> LightSpriteEvent {
        if self.is_dead() {
            self.death_state += 1;
            return LightSpriteEvent::None;
        }

        LightSpriteEvent::None
    }
}