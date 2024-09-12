use avian2d::prelude::LinearVelocity;
use bevy::{
    app::{App, FixedUpdate, PreUpdate},
    ecs::schedule::IntoSystemConfigs,
    math::{Vec2, Vec3Swizzles},
    prelude::{ButtonInput, KeyCode, Plugin, Query, Res, Transform, With},
};

use crate::structure::*;

pub struct PongActorPlugin;

impl Plugin for PongActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            ((handle_player_input, ai_movement), move_paddles).chain(),
        );
        app.add_systems(FixedUpdate, unstick_ball);
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
            let accel = exp_easeout(a_to_b.y / WIN_HEIGHT, AI_SKILL);

            // If the AI paddle is above the ball, move down, if it's below move up
            velocity.0.y = accel;
        }
    }
}

// Update position of pong ball
// fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
//     if let Ok((mut position, velocity)) = ball.get_single_mut() {
//         position.0 += velocity.0 * BALL_SPEED;
//     }
// }

fn unstick_ball(mut ball: Query<&mut LinearVelocity, With<Ball>>) {
    if let Ok(mut velocity) = ball.get_single_mut() {
        if velocity.0.x < 10. && velocity.0.y < 10. {
            velocity.0 = Vec2::new(
                BALL_SPEED * velocity.x.signum(),
                BALL_SPEED * velocity.y.signum(),
            );
        }
    }
}

fn move_paddles(mut paddle: Query<(&mut Transform, &Velocity), With<Paddle>>) {
    let max_y = WIN_HEIGHT / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;

    for (mut transform, velocity) in &mut paddle {
        let new_position = transform.translation.xy() + velocity.0 * PADDLE_SPEED;
        if new_position.y.abs() < max_y {
            transform.translation = new_position.extend(0.);
        }
    }
}
