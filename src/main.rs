pub mod components;
use components::*;
use std::borrow::{Borrow, BorrowMut};

pub mod constant;
use constant::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ScoreBoard { score: 0 })
        .insert_resource(SnakeManager::new())
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_UPDATE_TIME_NORMAL))
        .add_systems(Startup, (system_setup, system_log).chain())
        .add_systems(Update, system_handle_input)
        .add_systems(
            FixedUpdate,
            (system_update, system_update_scoreboard).chain(),
        )
        .run();
}

fn system_setup(mut commands: Commands, mut manager: ResMut<SnakeManager>) {
    // add the default camera
    commands.spawn(Camera2dBundle::default());

    // add the game scoreboard
    commands.spawn((
        ScoreBoardUI,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCOREBOARD_SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));

    // init the wall board
    {
        use components::Towards::*;
        [Left, Right, Up, Down].iter().for_each(|towards| {
            commands.spawn(WallBundle::new(towards));
        });
    }

    // summon a snake
    let entity = commands
        .spawn(SnakeEntity::new(SNAKE_DEFAULT_START_POSITION))
        .id();
    manager.borrow_mut().snake.r#move(entity);
}

/// TODO: Add a snake head
/// TODO: Add collision detective
/// TODO: Add translation animation
fn system_update(
    mut commands: Commands,
    mut manager: ResMut<SnakeManager>,
    mut score: ResMut<ScoreBoard>,
    query: Query<&Transform>
) {
    // add a new snake body
    let new_snake_posit = manager.borrow_mut().update_head_position();
    for entity in query.iter() {
        if entity.translation == new_snake_posit.translation() {
            println!("causing collider");
            break;
        }
    }
    let entity = commands.spawn(SnakeEntity::new(new_snake_posit)).id();
    


    // add reward
    if let Some((p, e)) = manager.borrow().reward {
        if p == new_snake_posit {
            manager.borrow_mut().snake.grow(1);
            manager.borrow_mut().reward = None;
            commands.entity(e).despawn();
            *score.score.borrow_mut() += 1;
        }
    }

    if let None = manager.borrow().reward {
        let posit = Position::random();
        let e = commands.spawn(RewardEntity::new(posit)).id();
        println!("generate a reward as {posit:?}, e: {e:?}");
        manager.borrow_mut().reward = Some((posit, e));
    }

    if let Some((towards, entity)) = manager.borrow_mut().snake.r#move(entity) {
        commands.entity(entity).despawn();
        manager.borrow_mut().update_tail_position(towards);
    }
}

fn system_handle_input(input: Res<ButtonInput<KeyCode>>, mut manager: ResMut<SnakeManager>) {
    [
        (KeyCode::ArrowLeft, Towards::Left),
        (KeyCode::ArrowRight, Towards::Right),
        (KeyCode::ArrowUp, Towards::Up),
        (KeyCode::ArrowDown, Towards::Down),
    ]
    .into_iter()
    .for_each(|(k, t)| {
        if input.pressed(k) {
            manager.borrow_mut().snake.change_towards(t);
        }
    })
}

fn system_log() {
    println!("The Game Board: r{BOARD_ROWS} x c{BOARD_COLS}");
    println!("The Board Size: w{BOARD_WIDTH} x h{BOARD_HEIGHT}");
}

fn system_update_scoreboard(
    score_board: Res<ScoreBoard>,
    mut query: Query<&mut Text, With<ScoreBoardUI>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = score_board.score.to_string();
}
