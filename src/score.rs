use crate::structure::*;
use avian2d::prelude::LinearVelocity;
use bevy::{
    app::{App, Update},
    ecs::schedule::IntoSystemConfigs,
    math::{Vec2, Vec3},
    prelude::{
        DetectChanges, EventReader, EventWriter, Plugin, Query, Res, ResMut, Text, Transform, With,
    },
};

pub struct PongScorePlugin;

impl Plugin for PongScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_event::<ScoreEvent>();
        app.add_systems(
            Update,
            (
                detect_scoring,
                update_score,
                (update_scoreboard, reset_ball),
            )
                .chain(),
        );
    }
}

// If the ball is off the screen, check which players side and issue a ScoreEvent with that information
fn detect_scoring(ball: Query<&Position, With<Ball>>, mut events: EventWriter<ScoreEvent>) {
    if let Ok(ball) = ball.get_single() {
        if ball.0.x > WIN_WIDTH / 2. {
            events.send(ScoreEvent(Scorer::Ai));
        } else if ball.0.x < -WIN_WIDTH / 2. {
            events.send(ScoreEvent(Scorer::Player));
        }
    }
}

// Listen for ScoreEvents and update global score accordingly
fn update_score(mut score: ResMut<Score>, mut events: EventReader<ScoreEvent>) {
    for event in events.read() {
        match event.0 {
            Scorer::Ai => score.ai += 1,
            Scorer::Player => score.player += 1,
        }
    }
}

// Return the ball to the center on a ScoreEvent
fn reset_ball(
    mut ball: Query<(&mut Transform, &mut LinearVelocity), With<Ball>>,
    mut events: EventReader<ScoreEvent>,
) {
    for event in events.read() {
        if let Ok((mut transform, mut velocity)) = ball.get_single_mut() {
            match event.0 {
                Scorer::Ai => {
                    transform.translation = Vec3::ZERO;
                    velocity.0 = Vec2::new(-BALL_SPEED, BALL_SPEED);
                }
                Scorer::Player => {
                    transform.translation = Vec3::ZERO;
                    velocity.0 = Vec2::new(BALL_SPEED, BALL_SPEED);
                }
            }
        }
    }
}

// When the score changes, update the UI score text
fn update_scoreboard(mut scoreboard: Query<&mut Text, With<Scoreboard>>, score: Res<Score>) {
    if score.is_changed() {
        if let Ok(mut text) = scoreboard.get_single_mut() {
            text.sections[0].value = format!("{} - {}", score.player, score.ai);
        }
    }
}
