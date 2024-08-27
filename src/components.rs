use std::{
    collections::VecDeque,
    ops::{Add, Mul},
};
use std::ops::Div;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use rand::Rng;

use crate::constant::*;

/// The Position is relative to the game board, the left top is (0, 0)
/// means the (row, col) grid position
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub val: Vec2,
}

impl Position {
    pub fn from_translation(trans: Vec3) -> Self {
        let v = 
            trans - vec3(WALL_LEFT + CELL_SIZE / 2., WALL_TOP - CELL_SIZE / 2., 0.);
        Position {
            val: v.div(CELL_SIZE).xy()
        }
    }
    
    pub fn translation(self) -> Vec3 {
        let v = vec2(self.val.y, -self.val.x);
        v.mul(CELL_SIZE).extend(0.)
            + vec3(WALL_LEFT + CELL_SIZE / 2., WALL_TOP - CELL_SIZE / 2., 0.)
    }

    pub fn random() -> Self {
        let r = rand::thread_rng().gen_range(0..BOARD_ROWS) as f32;
        let c = rand::thread_rng().gen_range(0..BOARD_ROWS) as f32;
        Position { val: vec2(r, c) }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Position {
        Position {
            val: self.val + rhs.val,
        }
    }
}

/// It marks the object that can make collision event
#[derive(Component, Debug)]
pub struct Collider;

#[derive(Component)]
pub struct ScoreBoardUI;

#[derive(Resource, Debug)]
pub struct ScoreBoard {
    pub score: u32,
}

#[derive(Bundle)]
pub struct WallBundle {
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
}

#[derive(Component, Debug)]
pub struct Cell;

#[derive(Component, Debug, Clone, Copy)]
pub enum Towards {
    Left,
    Right,
    Up,
    Down,
}

impl Towards {
    fn oppose(self, towards: Towards) -> bool {
        matches!(
            (self, towards),
            (Towards::Left, Towards::Right)
                | (Towards::Right, Towards::Left)
                | (Towards::Up, Towards::Down)
                | (Towards::Down, Towards::Up)
        )
    }

    pub fn delta(self) -> Position {
        Position {
            val: match self {
                Towards::Down => Vec2::new(1., 0.),
                Towards::Up => Vec2::new(-1., 0.),
                Towards::Left => Vec2::new(0., -1.),
                Towards::Right => Vec2::new(0., 1.),
            },
        }
    }
}

#[derive(Bundle)]
pub struct RewardEntity {
    spatial: SpriteBundle,
}

impl RewardEntity {
    pub fn new(position: Position) -> Self {
        Self {
            spatial: create_entity(position, CELL_SCALE, CellState::Reward.color()),
        }
    }
}

#[derive(Bundle)]
pub struct SnakeEntity { 
    spatial: SpriteBundle,
}

impl SnakeEntity {
    pub fn position(&self) -> Position {
        Position::from_translation(self.spatial.transform.translation)
    }
}

fn create_entity(position: Position, scale: Vec3, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: position.translation(),
            scale,
            ..default()
        },
        sprite: Sprite { color, ..default() },
        ..default()
    }
}

impl SnakeEntity {
    pub fn new(position: Position) -> Self {
        Self {
            spatial: create_entity(position, SNAKE_BODY_SCALE, CellState::Occupied.color()),
        }
    }
}

#[derive(Component, Debug)]
pub struct SnakeHead;

#[derive(Debug)]
pub struct Snake {
    pub head: Towards,
    pub body: VecDeque<(Towards, Entity)>,
    pub grow: usize,
}

#[derive(Resource, Debug)]
pub struct SnakeManager {
    pub snake: Snake,
    pub tail_position: Position,
    pub head_position: Position,
    pub reward: Option<(Position, Entity)>,
}

impl Default for SnakeManager {
    fn default() -> Self {
        Self {
            snake: Snake {
                grow: SNAKE_DEFAULT_LEN,
                head: SNAKE_DEFAULT_TOWARDS,
                body: VecDeque::new(),
            },
            head_position: SNAKE_DEFAULT_START_POSITION,
            tail_position: SNAKE_DEFAULT_START_POSITION,
            reward: None,
        }
    }
}

impl SnakeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_tail_position(&mut self, towards: Towards) -> Position {
        let new_snake_posit = self.tail_position + towards.delta();
        self.tail_position = new_snake_posit;
        new_snake_posit
    }

    pub fn update_head_position(&mut self) -> Position {
        let new_snake_posit = self.head_position + self.snake.head.delta();
        self.head_position = new_snake_posit;
        new_snake_posit
    }
}

impl Snake {
    /// The return value means whether to remove the last snake body
    /// TODO: consider rename this function
    pub fn r#move(&mut self, entity: Entity) -> Option<(Towards, Entity)> {
        self.body.push_front((self.head, entity));
        if self.grow > 0 {
            self.grow -= 1;
            None
        } else {
            self.body.pop_back()
        }
    }

    pub fn change_towards(&mut self, towards: Towards) {
        let next_towards = self.body.front().unwrap().0;
        if !towards.oppose(next_towards) {
            self.head = towards;
        }
    }

    pub fn grow(&mut self, len: usize) {
        self.grow += len;
    }
}

pub struct WallPosition;

impl WallPosition {
    pub fn position(towards: &Towards) -> Vec2 {
        match towards {
            Towards::Left => Vec2 {
                x: WALL_LEFT - WALL_BOARDER / 2.,
                y: WALL_BOARDER / 2.,
            },
            Towards::Right => Vec2 {
                x: WALL_RIGNT + WALL_BOARDER / 2.,
                y: -WALL_BOARDER / 2.,
            },
            Towards::Up => Vec2 {
                x: WALL_BOARDER / 2.,
                y: WALL_TOP + WALL_BOARDER / 2.,
            },
            Towards::Down => Vec2 {
                x: -WALL_BOARDER / 2.,
                y: WALL_BOLOW - WALL_BOARDER / 2.,
            },
        }
    }

    pub fn size(towards: &Towards) -> Vec2 {
        let width = WALL_RIGNT - WALL_LEFT;
        let height = WALL_TOP - WALL_BOLOW;
        match towards {
            Towards::Left | Towards::Right => Vec2 {
                x: WALL_BOARDER,
                y: height + WALL_BOARDER,
            },
            Towards::Up | Towards::Down => Vec2 {
                x: width + WALL_BOARDER,
                y: WALL_BOARDER,
            },
        }
    }
}

impl WallBundle {
    pub fn new(towards: &Towards) -> Self {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: WallPosition::position(towards).extend(0.),
                    scale: WallPosition::size(towards).extend(1.),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub enum CellState {
    Empty,
    Wall,
    Occupied,
    Reward,
}

impl CellState {
    pub fn color(&self) -> Color {
        match self {
            CellState::Empty => BACKGROUND_COLOR,
            CellState::Wall => WALL_COLOR,
            CellState::Occupied => SNAKE_BODY_COLOR,
            CellState::Reward => REWARD_COLOR,
        }
    }
}
