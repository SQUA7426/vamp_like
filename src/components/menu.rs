use bevy::prelude::*;

#[derive(Debug)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_menu);
    }
}

fn setup_menu() {}
