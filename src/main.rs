use bevy::prelude::*;
use vamp_like::components::{menu::MenuPlugin, player::PlayerPlugin};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((MenuPlugin, PlayerPlugin))
        .run();
}
