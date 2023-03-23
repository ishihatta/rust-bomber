use sdl2::keyboard::KeyboardState;

use crate::game_screen::{
    constants,
    bomb::Bomb,
    player_input::{PlayerInput, Movement},
    screen::{self, GameScreen},
    light_sprite::LightSprite, player_operation::PlayerOperation,
};

use super::{
    field::{Field, ELEMENT_SIZE},
    field_element::{FieldElement, FieldObject},
    ai_constants,
    position::Position,
};


pub struct AIPlayerAdditionalInfo {
    field: Field,
    player_position: Position,
    opponent_position: Position,
    player_power: i32,
    opponent_is_dead: bool,
}

impl AIPlayerAdditionalInfo {
    pub fn new(game_screen: &GameScreen, player_number: usize) -> AIPlayerAdditionalInfo {
        let player = &game_screen.players[player_number];
        let opponent = &game_screen.players[1 - player_number];
        AIPlayerAdditionalInfo {
            field: Field::from_game_screen(game_screen),
            player_position: Position { x: player.get_x(), y: player.get_y() },
            opponent_position: Position { x: opponent.get_x(), y: opponent.get_y() },
            player_power: player.power,
            opponent_is_dead: opponent.is_dead(),
        }
    }
}

pub struct AIPlayer {
    previous_my_position: Position,
    // 前フレームで移動しようとした場合は true
    previous_want_to_mode: bool,
    // 対戦相手のストレス度に対するスコアの重みにプラスする値
    opponent_stress_weight_plus: i32,
    // 対戦相手の位置を通過できないと認識するタイマー（1以上だと対戦相手の現在位置を「通過不能」と判断する）
    opponent_position_is_not_passable_timer: i32,
}

impl PlayerOperation for AIPlayer {
    fn get_player_input(&mut self, _: &KeyboardState, ai_additional_info: Option<AIPlayerAdditionalInfo>) -> PlayerInput {
        let mut info = ai_additional_info.unwrap();
        self.get_player_input(&mut info)
    }
}

impl AIPlayer {
    pub fn new() -> AIPlayer {
        AIPlayer {
            previous_my_position: Position {x: 0, y: 0},
            previous_want_to_mode: false,
            opponent_stress_weight_plus: 0,
            opponent_position_is_not_passable_timer: 0,
        }
    }

    pub fn get_player_input(&mut self, additional_info: &mut AIPlayerAdditionalInfo) -> PlayerInput {
        // 前フレームで移動したかったけど移動できなかった場合、対戦相手のストレス度に対するスコアの重みを増やす
        if self.previous_want_to_mode && self.previous_my_position == additional_info.player_position {
            self.opponent_stress_weight_plus += 1;
        } else {
            if self.opponent_stress_weight_plus > 0 {
                self.opponent_stress_weight_plus -= 1;
            }
        }
        self.previous_my_position = additional_info.player_position;

        // 相手プレイヤーの座標
        let opponent_x = (additional_info.opponent_position.x + constants::CHARACTER_SIZE / 2) / constants::CHARACTER_SIZE;
        let opponent_y = (additional_info.opponent_position.y + constants::CHARACTER_SIZE / 2) / constants::CHARACTER_SIZE;

        // マップを作成する
        // let mut field = Field::from_game_screen(game_screen);

        // 現在の対戦相手のストレス
        let opponent_stress = Self::calc_opponent_stress(&additional_info.field, opponent_x, opponent_y);

        // 広さ優先で自キャラの位置から探索
        let mut search_queue = Vec::<Position>::new();
        let my_x = (additional_info.player_position.x + constants::CHARACTER_SIZE / 2) / constants::CHARACTER_SIZE;
        let my_y = (additional_info.player_position.y + constants::CHARACTER_SIZE / 2) / constants::CHARACTER_SIZE;
        {
            let my_element = additional_info.field.get_element_mut(my_x, my_y);
            my_element.distance = 0;
            my_element.cost = 0;
        }
        let my_position = Position { x: my_x, y: my_y};
        let mut max_score = -additional_info.field.get_element(my_x, my_y).risk;
        let mut max_score_position = Position { x: my_x, y: my_y };
        let mut max_score_fire = false;
        search_queue.push(Position { x: my_x, y: my_y });
        while !search_queue.is_empty() {
            let field_position = search_queue.remove(0);
            let x = field_position.x;
            let y = field_position.y;
            let distance: i32;
            let cost: i32;
            let risk: i32;
            {
                let field_element = additional_info.field.get_element(x, y);
                distance = field_element.distance;
                cost = field_element.cost;
                risk = field_element.risk;

                // この場所のスコアと爆弾設置の可否を計算する
                let mut score = -field_element.risk - distance * ai_constants::SCORE_OF_DISTANCE;
                let mut fire = false;
                match field_element.field_object {
                    // パワーアップアイテムがある場所には行きたい！
                    FieldObject::PowerUpItem => {
                        score += ai_constants::SCORE_OF_POWER_UP_ITEM;
                    }
                    FieldObject::Bomb => (),
                    _ => {
                        // この場所に爆弾を置いて得られるメリットを計算する
                        if !(x == opponent_x && y == opponent_y) {
                            // 爆弾が置かれた状態を再現する
                            let mut field_if_bomb_set = additional_info.field.clone();
                            // この爆弾で破壊できる壁の数
                            let break_count = field_if_bomb_set.add_bomb(
                                &Bomb::new(x * constants::CHARACTER_SIZE, y * constants::CHARACTER_SIZE, additional_info.player_power)
                            );
                            // 逃げ場があるか確認する
                            if field_if_bomb_set.check_if_escapable(x, y, opponent_x, opponent_y) {
                                // 破壊できる壁があればスコア加算
                                if break_count > 0 {
                                    score += break_count * ai_constants::SCORE_OF_BREAK_WALL;
                                    fire = true;
                                }
                                // 対戦相手にいやがらせできればスコア加算
                                if !additional_info.opponent_is_dead {
                                    let opponent_stress_plus = Self::calc_opponent_stress(&field_if_bomb_set, opponent_x, opponent_y) - opponent_stress;
                                    if opponent_stress_plus > 0 {
                                        // スコアに加算する重みの計算
                                        // 「動きたいのに動けない」状況が続くと現在位置に爆弾を置く場合の重みが大きくなる
                                        let weight = ai_constants::OPPONENT_STRESS_WEIGHT +
                                                if field_position == my_position { self.opponent_stress_weight_plus } else { 0 };
                                        score += opponent_stress_plus * weight;
                                        fire = true;
                                    }
                                }
                            }
                        }
                    }
                }
                if score > max_score {
                    max_score = score;
                    max_score_position = field_position;
                    max_score_fire = fire;
                }
            }

            let next_distance = distance + 1;
            let next_positions = [
                Position { x: x - 1, y },
                Position { x: x + 1, y },
                Position { x, y: y - 1 },
                Position { x, y: y + 1 },
            ];
            for next_position in next_positions {
                let next_element = additional_info.field.get_element_mut(next_position.x, next_position.y);

                // 通れない場所には行けない
                if !next_element.is_passable() {
                    continue;
                }
                // 「対戦相手位置を通過不能と認識する」場合は、対戦相手位置は通過不能とする
                if self.opponent_position_is_not_passable_timer > 0 &&
                    opponent_x == next_element.x &&
                    opponent_y == next_element.y {
                    continue;
                }
                // この場所のリスクが高すぎる場合はここには行かない
                if next_element.risk > ai_constants::RISK_OF_BOMB * 9 / 10 && next_element.risk > risk {
                    continue;
                }
                // この場所にたどり着くまでのコストを計算し、すでにそれより低いコストで移動できる経路が計算済みなら何もしない
                let cost = cost + next_element.risk;
                if cost >= next_element.cost {
                    continue;
                }

                next_element.previous_element_x = x;
                next_element.previous_element_y = y;
                next_element.distance = next_distance;
                next_element.cost = cost;
                search_queue.push(next_position);
            }
        }

        // 移動処理
        // すでに目的地に到着しており、かつ目的が爆弾設置なら爆弾を置く
        let fire_flag = max_score_position == my_position && max_score_fire;
        if fire_flag {
            // 爆弾を設置する場合は「相手位置は通過不能と認識する」タイマーをセットする
            self.opponent_position_is_not_passable_timer = ai_constants::OPPONENT_NOT_PASSABLE_TIMEOUT;
        } else if self.opponent_position_is_not_passable_timer > 0 {
            self.opponent_position_is_not_passable_timer -= 1;
        }
        // 目的地への経路のうち、現在地の次の位置を取得する
        let mut f = additional_info.field.get_element(max_score_position.x, max_score_position.y);
        while f.previous_element_x != my_x || f.previous_element_y != my_y {
            if f.previous_element_x < 0 {
                break;
            }
            f = additional_info.field.get_element(f.previous_element_x, f.previous_element_y);
        }
        let fx = f.x * constants::CHARACTER_SIZE;
        let fy = f.y * constants::CHARACTER_SIZE;
        self.previous_want_to_mode = fx != additional_info.player_position.x || fy != additional_info.player_position.y;
        if fx > additional_info.player_position.x {
            // 右に移動
            PlayerInput::new(Movement::RIGHT, fire_flag)
        } else if fx < additional_info.player_position.x {
            // 左に移動
            PlayerInput::new(Movement::LEFT, fire_flag)
        } else if fy < additional_info.player_position.y {
            // 上に移動
            PlayerInput::new(Movement::UP, fire_flag)
        } else if fy > additional_info.player_position.y {
            // 下に移動
            PlayerInput::new(Movement::DOWN, fire_flag)
        } else {
            PlayerInput::new(Movement::NONE, fire_flag)
        }
    }

    /// 対戦相手の移動範囲のうち何パーセントを「いずれ爆発する」状態にしているか
    fn calc_opponent_stress(field: &Field, opponent_x: i32, opponent_y: i32) -> i32 {
        // 到達可能で、かつ距離が5以下の場所を探索する
        let mut checked = [false; ELEMENT_SIZE];
        let mut search_queue = Vec::<&FieldElement>::new();
        search_queue.push(field.get_element(opponent_x, opponent_y));
        checked[(opponent_x + opponent_y * screen::MAP_WIDTH) as usize] = true;
        // 移動可能な範囲
        let mut movable_space = 0;
        // 危険な範囲
        let mut dangerous_space = 0;
        while !search_queue.is_empty() {
            let element = search_queue.remove(0);
            let ex = element.x;
            let ey = element.y;
            movable_space += 1;
            if element.risk > 0 {
                dangerous_space += 1;
            }
            let next_elements = [
                field.get_element(ex - 1, ey),
                field.get_element(ex + 1, ey),
                field.get_element(ex, ey - 1),
                field.get_element(ex, ey + 1),
            ];
            for next_element in next_elements {
                let idx = (next_element.x + next_element.y * screen::MAP_WIDTH) as usize;
                let distance = (opponent_x - next_element.x).abs() + (opponent_y - next_element.y).abs();
                if distance <= 5 && !checked[idx] && next_element.is_passable() {
                    search_queue.push(next_element);
                    checked[idx] = true;
                }
            }
        }
        dangerous_space * 100 / movable_space
    }
}