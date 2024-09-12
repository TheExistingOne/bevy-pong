use bevy::{
    app::{App, FixedUpdate},
    math::Vec3Swizzles,
    prelude::{Plugin, Query, Transform},
};

use crate::structure::*;

pub struct PongGameStatePlugin;

impl Plugin for PongGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, project_positions);
    }
}

// Map 2d Position to engine-internal Transform
fn project_positions(mut positionables: Query<(&Transform, &mut Position)>) {
    for (transform, mut position) in &mut positionables {
        position.0 = transform.translation.xy();
    }
}
