use bevy::prelude::Vec3;

pub trait Character {
    fn new(char_name: String, hp: f32) -> Self;

    fn spawnpoint(&self, player_pos: Vec3) -> Vec3;

    fn attack(&self);
}
