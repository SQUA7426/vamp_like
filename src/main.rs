use bevy::prelude::*;
use vamp_like::components::{cam::CamPlugin, enemy::EnemyPlugin, level::LevelPlugin, menu::MenuPlugin, player::PlayerPlugin};

fn main() {
    // new v.0.2.0
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CamPlugin, EnemyPlugin, LevelPlugin, MenuPlugin, PlayerPlugin))
        .run();
}
