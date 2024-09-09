use bevy::{
    app::{App, Startup},
    asset::Assets,
    ecs::{
        change_detection::ResMut,
        system::{Commands, Query},
    },
    prelude::{
        default, Camera2dBundle, Circle, Color, ColorMaterial, JustifyText, Mesh, Plugin,
        Rectangle, Text, Text2dBundle, TextStyle, Transform, Window,
    },
    sprite::{Anchor, MaterialMesh2dBundle},
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let ball = BallBundle::new(1., 1.);

    // Define ball mesh and material
    let shape = Mesh::from(Circle::new(ball.shape.0.x));
    let color = ColorMaterial::from(Color::hsl(0., 1., 0.28));

    // Instance mesh into memory and return Handle reference
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
        ball,
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
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
        let color = ColorMaterial::from(Color::hsl(105., 1., 0.28));

        // Instance mesh into memory and return Handle reference
        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);

        commands.spawn((
            Player,
            player_paddle,
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                ..default()
            },
        ));

        commands.spawn((
            Ai,
            ai_paddle,
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
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
    let material = ColorMaterial::from(Color::hsl(0., 1., 1.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        top_gutter,
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            ..default()
        },
    ));

    commands.spawn((
        bottom_gutter,
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
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
