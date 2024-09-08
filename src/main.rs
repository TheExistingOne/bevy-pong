use bevy::{app::PluginGroupBuilder, prelude::*};

mod structure; // Stores Components, Events, Enums, and other building blocks
mod setup; // Contains initialization information
mod score; // Scores scoring elements
mod actors; // Stores behavior of active objects
mod gamestate; // Stores core game functionality like collisions and internal-to-engine mapping

struct PongGame;
impl PluginGroup for PongGame {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::PongInitPlugin)
            .add(score::PongScorePlugin)
            .add(actors::PongActorPlugin)
            .add(gamestate::PongGameStatePlugin)
    }
}

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        PongGame
    )).run();
}