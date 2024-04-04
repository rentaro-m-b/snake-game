use crate::point::Point;
use crate::config::{GRID_X_SIZE, GRID_Y_SIZE};
use rand::{thread_rng, Rng};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Food {
    pub positions: Vec<Point>, // 食べ物の位置
}

impl Food {
    // 新しい食べ物のインスタンスを生成する
    pub fn new() -> Self {
        let mut foods = Vec::new();

        let mut rng = thread_rng();
        for _ in 0..3 {
            let food_x = rng.gen_range(0..GRID_X_SIZE);
            let food_y = rng.gen_range(0..GRID_Y_SIZE);
            foods.push(Point(food_x, food_y));
        }

        Food { positions: foods }
    }

    // 食べ物をランダムに動かす
    pub fn move_food(&mut self) {
        let mut rng = thread_rng();
        for food in self.positions.iter_mut() {
            let direction = rng.gen_range(0..4); // 0から3の間の乱数を生成
            match direction {
                0 => food.1 = food.1.saturating_sub(1), // 上に移動
                1 => food.1 = (food.1 + 1).min(GRID_Y_SIZE - 1), // 下に移動
                2 => food.0 = food.0.saturating_sub(1), // 左に移動
                3 => food.0 = (food.0 + 1).min(GRID_X_SIZE - 1), // 右に移動
                _ => {}
            }
        }
    }

    // 食べ物を追加する
    pub fn add_food(&mut self) {
        let mut rng = thread_rng();
        let food_x = rng.gen_range(0..GRID_X_SIZE);
        let food_y = rng.gen_range(0..GRID_Y_SIZE);
        self.positions.push(Point(food_x, food_y));
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RED);
        for position in &self.positions {
            let rect = Rect::new(position.0 * 20, position.1 * 20, 20, 20); // 同じく20x20の矩形
            canvas.fill_rect(rect)?;
        }
        Ok(())
    }
}
