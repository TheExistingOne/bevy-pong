use avian2d::prelude::Gravity;
use bevy::{
    app::{App, Startup},
    asset::{AssetServer, Assets},
    ecs::{
        change_detection::ResMut,
        system::{Commands, Query},
    },
    math::Vec2,
    prelude::{
        default, Camera2dBundle, Color, ColorMaterial, JustifyText, Mesh, Plugin, Rectangle, Res,
        Text, Text2dBundle, TextStyle, Transform, Window,
    },
    sprite::{Anchor, MaterialMesh2dBundle, Sprite, SpriteBundle},
};

use crate::structure::*;

pub struct PongInitPlugin;

impl Plugin for PongInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_camera,
                spawn_ball,
                spawn_paddles,
                spawn_gutters,
                spawn_scoreboard,
            ),
        );
        app.insert_resource(Gravity(Vec2::ZERO));
    }
}

// ##############################################################
// # Setup
// ##############################################################

// Create scene camera
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

// Spawn a red pong ball at the center of the screen moving up-right
pub fn spawn_ball(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    println!("Spawning ball...");

    let ball = BallBundle::new(150., 150.);

    // Define ball mesh and material
    //let shape = Mesh::from(Circle::new(ball.shape.0.x));
    //let color = ColorMaterial::from(Color::hsl(0., 1., 0.28));

    // Instance mesh into memory and return Handle reference
    //let mesh_handle = meshes.add(shape);
    //let material_handle = materials.add(color);

    commands.spawn((
        ball,
        SpriteBundle {
            texture: asset_server.load("ball.png"),
            sprite: Sprite {
                custom_size: Some((BALL_SIZE, BALL_SIZE).into()),
                ..default()
            },
            ..default()
        }, // MaterialMesh2dBundle {
           //     mesh: mesh_handle.into(),
           //     material: material_handle,
           //     transform: Transform::from_translation(Vec3::ZERO),
           //     ..default()
           // },
    ));
}

// Spawn Player and AI paddles at the left and right edge of the screen
pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddles...");

    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        let player_paddle = PaddleBundle::new(right_paddle_x, 0.);
        let ai_paddle = PaddleBundle::new(left_paddle_x, 0.);

        let shape = Mesh::from(Rectangle::new(ai_paddle.shape.0.x, ai_paddle.shape.0.y));
        let color = ColorMaterial::from(Color::WHITE);

        // Instance mesh into memory and return Handle reference
        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);

        commands.spawn((
            Player,
            player_paddle,
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(right_paddle_x, 0., 0.),
                ..default()
            },
        ));

        commands.spawn((
            Ai,
            ai_paddle,
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                transform: Transform::from_xyz(left_paddle_x, 0., 0.),
                ..default()
            },
        ));
    }
}

// Spawn gutters at the top and bottom of the screen
pub fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Window coordinates have the corner be 0,0 while world coordinates that's the center so
    // we devide by two to convert
    let top_gutter_y = WIN_HEIGHT / 2. - GUTTER_HEIGHT / 2.;
    let bottom_gutter_y = -WIN_HEIGHT / 2. + GUTTER_HEIGHT / 2.;

    let top_gutter = GutterBundle::new(0., top_gutter_y, WIN_WIDTH);
    let bottom_gutter = GutterBundle::new(0., bottom_gutter_y, WIN_WIDTH);

    let mesh = Mesh::from(Rectangle::from_size(top_gutter.shape.0));
    let material = ColorMaterial::from(Color::WHITE);

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        top_gutter,
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            transform: Transform::from_xyz(0., top_gutter_y, 0.),
            ..default()
        },
    ));

    commands.spawn((
        bottom_gutter,
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_xyz(0., bottom_gutter_y, 0.),
            ..default()
        },
    ));
}

// Spawn a scoreboard displaying the game score in the format {ai} - {player} at the top of the screen
pub fn spawn_scoreboard(mut commands: Commands) {
    let text_pos = Transform::from_xyz(0., (WIN_HEIGHT / 2.) - 20., 0.);

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0 - 0",
                TextStyle {
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            text_anchor: Anchor::TopCenter,
            transform: text_pos,
            ..default()
        },
        Scoreboard,
    ));
}
