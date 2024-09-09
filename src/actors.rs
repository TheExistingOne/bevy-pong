use bevy::{
    app::{App, PreUpdate},
    ecs::schedule::IntoSystemConfigs,
    prelude::{ButtonInput, KeyCode, Plugin, Query, Res, With},
};

use crate::structure::*;

pub struct PongActorPlugin;

impl Plugin for PongActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            ((handle_player_input, ai_movement), move_paddles).chain(),
        );
        app.add_systems(PreUpdate, move_ball);
    }
}

// Check for up/down input from player
fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

// Calculate AI movement direction (if any)
fn ai_movement(
    mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((mut velocity, position)) = ai.get_single_mut() {
        if let Ok(ball_position) = ball.get_single() {
            let a_to_b = ball_position.0 - position.0;

            // If the AI paddle is above the ball, move down, if it's below move up
            velocity.0.y = a_to_b.y.signum();
        }
    }
}

// Update position of pong ball
fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

fn move_paddles(mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>) {
    let max_y = WIN_HEIGHT / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;

    for (mut position, velocity) in &mut paddle {
        let new_position = position.0 + velocity.0 * PADDLE_SPEED;
        if new_position.y.abs() < max_y {
            position.0 = new_position;
        }
    }
}
