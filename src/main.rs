use bevy::prelude::*;
use vamp_like::components::{cam::CamPlugin, level::LevelPlugin, menu::MenuPlugin, player::PlayerPlugin};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CamPlugin, LevelPlugin, MenuPlugin, PlayerPlugin))
        .run();
}
