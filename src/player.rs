use crate::point::Point;
use crate::config::{GRID_X_SIZE, GRID_Y_SIZE};
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// プレイヤーの方向を示す列挙型
pub enum PlayerDirection { Up, Down, Right, Left }

pub struct Player {
    pub positions: Vec<Point>, // プレイヤーの位置（頭からシッポへの順序）
    pub direction: PlayerDirection, // 現在の移動方向
}

impl Player {
    pub fn new(initial_position: Vec<Point>, initial_direction: PlayerDirection) -> Self {
        Self {
            positions: initial_position,
            direction: initial_direction,
        }
    }

    // WASDキーに応じてプレイヤーの方向を更新する
    pub fn update_direction(&mut self, new_direction: PlayerDirection) {
        // 現在の方向と逆方向への移動を防ぐなど、追加のロジックをここに実装できます
        self.direction = new_direction;
    }

    // プレイヤーの位置を更新する
    pub fn move_player(&mut self) {
        let head_position = self.positions.first().expect("Player has no positions").clone();
        let mut new_head_position = match self.direction {
            PlayerDirection::Up => Point(head_position.0, head_position.1.saturating_sub(1)),
            PlayerDirection::Down => Point(head_position.0, (head_position.1 + 1).min(GRID_Y_SIZE - 1)),
            PlayerDirection::Right => Point((head_position.0 + 1).min(GRID_X_SIZE - 1), head_position.1),
            PlayerDirection::Left => Point(head_position.0.saturating_sub(1), head_position.1),
        };

        // 画面外に移動しないようにするためのチェック
        new_head_position.0 = new_head_position.0.clamp(0, GRID_X_SIZE - 1);
        new_head_position.1 = new_head_position.1.clamp(0, GRID_Y_SIZE - 1);

        // 古いシッポを削除し、新しい頭の位置を先頭に挿入
        self.positions.pop();
        self.positions.insert(0, new_head_position);
    }

    pub fn handle_input(&mut self, keycode: Keycode) {
        let new_direction = match keycode {
            Keycode::W => Some(PlayerDirection::Up),
            Keycode::A => Some(PlayerDirection::Left),
            Keycode::S => Some(PlayerDirection::Down),
            Keycode::D => Some(PlayerDirection::Right),
            _ => None,
        };

        if let Some(direction) = new_direction {
            self.update_direction(direction);
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::GREEN);
        for position in &self.positions {
            let rect = Rect::new(position.0 * 20, position.1 * 20, 20, 20); // 20x20の矩形
            canvas.fill_rect(rect)?;
        }
        Ok(())
    }
}
