use bevy::prelude::*;

use crate::components::size::Size;

pub struct Player {
    name: String,
    health: f32,
    speed: f32,
    size: Size,
    position: Vec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
    }
}

fn player_setup() {
}
