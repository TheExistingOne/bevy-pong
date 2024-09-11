use bevy::{
    app::{App, FixedUpdate},
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::{IntoSystemConfigs, Plugin, Query, Transform, Vec2, With, Without},
};

use crate::structure::*;

pub struct PongGameStatePlugin;

impl Plugin for PongGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (handle_collisions, project_positions).chain());
    }
}

// Map 2d Position to engine-internal Transform
fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

// Check for intersections between a circle collider (pong ball) and a provided axis-aligned bounding box
// Returns a tuple of the collision side and the position of the closest-ish point fully outside the collision
fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<(Collision, Vec2)> {
    // We're only doing discrete collision so if the bounding boxes don't intersect we don't care
    if !ball.intersects(&wall) {
        return None;
    }

    // Find the closest point on the ball collider to the wall
    let closest_point = wall.closest_point(ball.center);
    // Get the distance from the center to the collision point
    let offset = ball.center - closest_point;

    /*
    This collision check works like process of elimination. First we check whether we're closer to
    the sides or the top and bottom. That eliminates half the collision options immediately. Then
    we check if we're clipping the left or top wall (a negative collision distance). If we are, we
    return that, otherwise we return the opposite.
    */
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            (
                Collision::Left,
                closest_point - (Vec2::X * (ball.radius() + 0.5)),
            )
        } else {
            (
                Collision::Right,
                closest_point + (Vec2::X * (ball.radius() + 0.5)),
            )
        }
    } else if offset.y > 0. {
        (
            Collision::Top,
            closest_point + (Vec2::Y * (ball.radius() + 0.5)),
        )
    } else {
        (
            Collision::Bottom,
            closest_point - (Vec2::Y * (ball.radius() + 0.5)),
        )
    };

    Some(side)
}

// Check collisions between the pong ball and any Entity with a Shape Component using collide_with_side()
// Unlike most systems impacting game elements, this takes direct conotrol over Transform
fn handle_collisions(
    mut ball: Query<(&mut Velocity, &mut Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, mut ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some((collision, exit)) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.),
            ) {
                // Vertical distance to center of collision
                //let center_dist = (position.0.y - exit.y).abs();

                // Mapping distance to center to move direction
                //let mapped_angle = f32_map(0., shape.0.y / 2., 0.5, 1.5, center_dist);

                match collision {
                    Collision::Left => {
                        ball_position.0 = exit;
                        //ball_velocity.0.y = mapped_angle * ball_velocity.0.y.signum();
                        ball_velocity.0.x = -1.;
                    }
                    Collision::Right => {
                        ball_position.0 = exit;
                        //ball_velocity.0.y = mapped_angle * ball_velocity.0.y.signum();
                        ball_velocity.0.x = -1.;
                    }
                    Collision::Top => {
                        ball_position.0 = exit;
                        ball_velocity.0.y *= -1.;
                    }
                    Collision::Bottom => {
                        ball_position.0 = exit;
                        ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}
