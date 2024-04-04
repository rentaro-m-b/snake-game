use crate::point::Point;
use crate::player::Player;
use crate::food::Food;
use sdl2::keyboard::Keycode;

// ゲーム状態（ポーズ中やプレイ中）を示す列挙型
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum GameState { Playing, Paused }

// プレイヤーの場所（蛇なので、それぞれの位置）、プレイヤーの方向、フードの位置、ゲームの状態を示す
pub struct GameContext {
    pub player: Player,
    pub foods: Food,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player: Player::new(vec![Point(3, 1), Point(2, 1), Point(1, 1)], crate::player::PlayerDirection::Right),
            foods: Food::new(),
            state: GameState::Paused,
        }
    }

    pub fn next_tick(&mut self) {
        if self.state == GameState::Paused {
            return;
        }
        
        // プレイヤーの位置を更新
        self.player.move_player();
    
        // 食べ物を動かす
        self.foods.move_food();
    }
    
    // ポーズとプレイを切り替える処理
    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing
        }
    }

    pub fn handle_input(&mut self, keycode: Keycode) {
        if keycode == Keycode::Escape {
            self.toggle_pause(); // エスケープキーでポーズとプレイの状態を切り替える
        }
    }
}
