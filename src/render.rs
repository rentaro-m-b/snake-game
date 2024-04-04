use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use crate::game_context::{GameState, GameContext};

pub struct Render {
    canvas: WindowCanvas,
}

impl Render {
    pub fn new(canvas: WindowCanvas) -> Self {
        Render { canvas }
    }

    pub fn draw_game(&mut self, context: &GameContext) -> Result<(), String> {
        // 背景色を設定
        let background_color = match context.state {
            GameState::Playing => Color::RGB(0, 0, 0), // 黒
            GameState::Paused => Color::RGB(105, 105, 105), // 暗い灰色
        };
        self.canvas.set_draw_color(background_color);
        self.canvas.clear();

        // プレイヤーを描画
        context.player.draw(&mut self.canvas)?;

        // 食べ物を描画
        context.foods.draw(&mut self.canvas)?;

        // 画面の更新
        self.canvas.present();

        Ok(())
    }
}
