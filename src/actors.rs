use avian2d::prelude::{CollisionEnded, LinearVelocity};
use bevy::{
    app::{App, FixedUpdate, PreUpdate},
    ecs::schedule::IntoSystemConfigs,
    math::{vec2, Vec2, Vec3Swizzles},
    prelude::{ButtonInput, Entity, EventReader, KeyCode, Plugin, Query, Res, Transform, With},
};

use crate::structure::*;

pub struct PongActorPlugin;

impl Plugin for PongActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            ((handle_player_input, ai_movement), move_paddles).chain(),
        );
        app.add_systems(FixedUpdate, (unstick_ball, reflect_ball));
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

fn reflect_ball(
    mut ball: Query<(Entity, &mut LinearVelocity, &Position), With<Ball>>,
    paddle: Query<(Entity, &Position), With<Paddle>>,
    mut events: EventReader<CollisionEnded>,
) {
    // Get a list of collisions since the last FixedUpdate and the entities involved
    for CollisionEnded(entity1, entity2) in events.read() {
        if let Ok((entity_ball, mut ball_vel, ball_pos)) = ball.get_single_mut() {
            for (entity_paddle, paddle_pos) in paddle.iter() {
                // I am aware this is a cursed abomination that can probably be better
                // Check if the two entities involved were a ball and a paddle
                if (entity1.index() == entity_ball.index()
                    || entity2.index() == entity_ball.index())
                    && (entity1.index() == entity_paddle.index()
                        || entity2.index() == entity_paddle.index())
                {
                    // How far from the center of the paddle did the ball hit? (0 = center, 25 = corner pixel)
                    let dist_from_center = (paddle_pos.0.y - ball_pos.0.y).abs();

                    // Remap that to a hit velocity between BALL_SPEED and 1.5 times BALL_SPEED
                    let scaled_dist = f32_map(
                        0.,
                        PADDLE_HEIGHT / 2.,
                        BALL_SPEED,
                        BALL_SPEED * 1.5,
                        dist_from_center,
                    );

                    // Convert that to a full velocity, respecting conservation of energy by scaling down the horizontal velocity accordingly
                    // This makes the hit behavior feel less weird
                    let hit_velocity =
                        vec2((BALL_SPEED * 2.) - scaled_dist, scaled_dist) * ball_vel.signum();

                    // Apply that velocity to the ball
                    ball_vel.0 = hit_velocity;
                }
            }
        }
    }
}

fn unstick_ball(mut ball: Query<&mut LinearVelocity, With<Ball>>) {
    if let Ok(mut velocity) = ball.get_single_mut() {
        // If the player or AI moves weirdly the ball can get pinched between the paddle and the bumper
        // This detects if the horizontal and vertical velocity get below 10, and resets them to default just in case
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
