use bevy::{
    ecs::{component::Component, event::Event},
    math::Vec2,
    prelude::{Bundle, Resource},
};

// ##############################################################
// # Global Constants
// ##############################################################

pub const WIN_HEIGHT: f32 = 720.;
pub const WIN_WIDTH: f32 = 1280.;

pub const BALL_SIZE: f32 = 5.; // Size of the ball in world units
pub const BALL_SPEED: f32 = 3.; // Speed per frame of the ball in world units

pub const PADDLE_SPEED: f32 = 5.; // Speed per frame of the paddles in world units
pub const PADDLE_WIDTH: f32 = 10.; // Dimensions of the paddles in world units
pub const PADDLE_HEIGHT: f32 = 50.;

pub const AI_SKILL: f32 = 15.; // Adjusts how rapidly the AI's smoothing function responds

pub const GUTTER_HEIGHT: f32 = 20.; // Height of the top and bottom gutters in world units

// ##############################################################
// # Helper Types
// ##############################################################

// Object collision sides
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Used to communicate which player scored in ScoreEvent
pub enum Scorer {
    Ai,
    Player,
}

// ##############################################################
// # Global Events & Resources
// ##############################################################

// Notify systems that the ball has been lost and by who
#[derive(Event)]
pub struct ScoreEvent(pub Scorer);

// Global store of game score
#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

// ##############################################################
// # Generic Components
// ##############################################################

// 2D object position, mapped internally to object transform by project_positions()
#[derive(Component, Default)]
pub struct Position(pub Vec2);

// 2D "velocity" - effectively a measure of what direction all the systems want an object to go
#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

// Generic component holding the height and width of an entity's primitive mesh
#[derive(Component)]
pub struct Shape(pub Vec2);

// ##############################################################
// # Entity Labels
// ##############################################################

// Player and AI Paddles
#[derive(Component)]
pub struct Ai;
#[derive(Component)]
pub struct Player;

// Pong Ball
#[derive(Component, Default)]
pub struct Ball;

// Any player's paddle
#[derive(Component, Default)]
pub struct Paddle;

// Top and bottom gutters
#[derive(Component, Default)]
pub struct Gutter;

// Score display
#[derive(Component)]
pub struct Scoreboard;

// ##############################################################
// # Entity Bundles
// ##############################################################

// Pong ball template
#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,         // Label
    pub shape: Shape,       // Dimensions of the sprite
    pub position: Position, // 2d position
    pub velocity: Velocity, // Sum of system forces/move direction
}

impl BallBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::ZERO),
        }
    }
}

// Generic paddle template
#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub shape: Shape,
    pub position: Position,
    pub velocity: Velocity,
}

impl PaddleBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::ZERO),
        }
    }
}

// Generic gutter template. No velocity cause they're static
#[derive(Bundle)]
pub struct GutterBundle {
    pub gutter: Gutter,
    pub shape: Shape,
    pub position: Position,
}

impl GutterBundle {
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        Self {
            gutter: Gutter,
            shape: Shape(Vec2::new(width, GUTTER_HEIGHT)),
            position: Position(Vec2::new(x, y)),
        }
    }
}

// ##############################################################
// # Helper Functions
// ##############################################################

// https://easings.net is a great resource for these
pub fn exp_easeout(t: f32, m: f32) -> f32 {
    // 1 - (1 - time)^magnitude
    1. - (1. - t).powf(m)
}

// Maps range of f32s to each other courtesy of https://stackoverflow.com/questions/5731863/mapping-a-numeric-range-onto-another
#[allow(dead_code)]
pub fn f32_map(orig_start: f32, orig_end: f32, new_start: f32, new_end: f32, to_map: f32) -> f32 {
    new_start + ((new_end - new_start) / (orig_end - orig_start)) * (to_map - orig_start)
}
