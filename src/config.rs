// ゲーム画面のグリッドサイズとドットのサイズを定義
pub const GRID_X_SIZE: i32 = 40;
pub const GRID_Y_SIZE: i32 = 30;
pub const DOT_SIZE_IN_PXS: i32 = 20;

// ゲーム画面全体のピクセルサイズを計算
pub const MAX_WIDTH: i32 = GRID_X_SIZE * DOT_SIZE_IN_PXS;
pub const MAX_HEIGHT: i32 = GRID_Y_SIZE * DOT_SIZE_IN_PXS;
