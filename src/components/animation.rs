use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_idx: usize,
    pub last_sprite_idx: usize,
    pub fps: u8,
    pub frame_timer: Timer,
    pub is_playing: bool,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps_: u8) -> Self {
        Self {
            first_sprite_idx: first,
            last_sprite_idx: last,
            fps: fps_,
            frame_timer: Self::timer_from_fps(fps_),
            is_playing: false,
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1. / (fps as f32)), TimerMode::Repeating)
    }
}

