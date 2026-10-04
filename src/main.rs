use bevy::prelude::*;
use vamp_like::components::{cam::CamPlugin, enemy::EnemyPlugin, level::LevelPlugin, menu::MenuPlugin, player::PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CamPlugin, EnemyPlugin, LevelPlugin, MenuPlugin, PlayerPlugin))
        .run();
}
