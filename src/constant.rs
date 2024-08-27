use bevy::prelude::*;

use crate::{Position, Towards};

pub const BOARD_ROWS: usize = 30;
pub const BOARD_COLS: usize = 40;
pub const BOARD_WIDTH: f32 = BOARD_COLS as f32 * CELL_SIZE;
pub const BOARD_HEIGHT: f32 = BOARD_ROWS as f32 * CELL_SIZE;

pub const WALL_RIGNT: f32 = BOARD_WIDTH / 2.;
pub const WALL_LEFT: f32 = -WALL_RIGNT;
pub const WALL_TOP: f32 = BOARD_HEIGHT / 2.;
pub const WALL_BOLOW: f32 = -WALL_TOP;
pub const WALL_BOARDER: f32 = CELL_SIZE;
pub const WALL_COLOR: Color =  Color::Srgba(Srgba::rgb(0.8, 0.8, 0.8));

pub const CELL_SIZE: f32 = 20.;
pub const CELL_SCALE: Vec3 = Vec3::new(CELL_SIZE + 1., CELL_SIZE + 1., 1.); // a bit larger to stand out

pub const SNAKE_BODY_SIZE: f32 = 19.; // a bit smaller to show distance
pub const SNAKE_BODY_SCALE: Vec3 = Vec3::new(SNAKE_BODY_SIZE, SNAKE_BODY_SIZE, 1.);
pub const SNAKE_DEFAULT_LEN: usize = 6;
pub const SNAKE_DEFAULT_TOWARDS: Towards = Towards::Right;
pub const SNAKE_DEFAULT_START_POSITION: Position = Position { val: Vec2::ZERO };

pub const SNAKE_BODY_COLOR: Color = Color::Srgba(Srgba::rgb(0.2, 0.6, 0.4));
pub const BACKGROUND_COLOR: Color = Color::Srgba(Srgba::rgb(0.1, 0.1, 0.1));
pub const REWARD_COLOR: Color = Color::Srgba(Srgba::rgb(0.5, 0.3, 0.2));

pub const FIXED_UPDATE_TIME_NORMAL: f64 = 0.09;

pub const SCOREBOARD_FONT_SIZE: f32 = 40.;
pub const SCOREBOARD_TEXT_COLOR: Color = Color::Srgba(Srgba::rgb(0.2, 0.6, 0.4));
pub const SCOREBOARD_SCORE_COLOR: Color = Color::Srgba(Srgba::rgb(0.8, 0.3, 0.2));
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
