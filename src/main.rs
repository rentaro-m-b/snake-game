use sdl2::event::Event;
use std::time::Duration;
use snake_game::game_context::GameContext;
use snake_game::render::Render;
use sdl2::pixels::Color;
use snake_game::config::{MAX_HEIGHT, MAX_WIDTH};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?; // SDL2の初期化
    let video_subsystem = sdl_context.video()?; // ビデオサブシステムの取得

    // ウィンドウの作成
    let window = video_subsystem.window("Snake Game", MAX_WIDTH as u32, MAX_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // キャンバスの作成
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(255, 255, 255)); // 描画色の設定

    let mut event_pump = sdl_context.event_pump()?; // イベントポンプの取得

    let mut game_context = GameContext::new(); // ゲームコンテキストの初期化
    let mut render = Render::new(canvas); // レンダラーの初期化

    let mut frame_count = 0;
    'running: loop {
        // イベントのポーリング
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    game_context.handle_input(keycode); // キー入力をGameContextに渡して処理
                    game_context.player.handle_input(keycode);
                },
                _ => {}
            }
        }
        if frame_count % 5 == 0 {
            game_context.next_tick();
        }
        frame_count += 1;

        render.draw_game(&game_context)?; // ゲームの状態を描画

        // フレームレートの制御（例：60FPS）
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
