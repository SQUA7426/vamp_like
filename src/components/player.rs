use bevy::prelude::*;
use bevy::color::palettes::css::BLUE_VIOLET;

use crate::components::menu::GameState;
use crate::components::size::Size;
use crate::traits::character::Character;

#[allow(unused)]
#[derive(Component, Debug)]
pub struct Player {
    name: String,
    attack: f32,
    sp_attack: f32,
    attack_speed: f32,
    defense: f32,
    sp_defense: f32,
    speed: f32,
    health: f32,
    size: Size,
    pub pos: Vec3,
}

impl Character for Player {
    fn new(char_name: String, hp: f32) -> Self {
        Self {
            name: char_name,
            attack: 5.0,
            sp_attack: 5.0,
            attack_speed: 5.0,
            defense: 5.0,
            sp_defense: 5.0,
            speed: 100.0,
            health: hp,
            size: Size::default(),
            pos: Vec3::ZERO,
        }
    }

    fn spawnpoint(&self, player_pos: Vec3) -> Vec3 {
        player_pos
    }
}

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), player_setup)
            .add_systems(Update, control_player.run_if(in_state(GameState::Playing)));
    }
}

fn player_setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let player = Player::new("Anton".to_string(), 100.0);
    cmds.spawn((
            Mesh2d(meshes.add(Circle::new(24.0))),
            MeshMaterial2d(materials.add(Color::from(BLUE_VIOLET))),
            Transform::from_translation(player.spawnpoint(player.pos)),
            player,
    ));
}


fn control_player(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Player, &mut Transform)>,
    time: Res<Time>
) {
    let (player, mut transform) = player.into_inner();

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let movement_delta = direction.normalize_or_zero() * player.speed * time.delta_secs();
    transform.translation += movement_delta.extend(0.);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_player() {
        let player = Player::new("Anton".to_string(), 100.0);

        assert_eq!(player.name, "Anton".to_string());
        assert_eq!(player.attack, 5.0);
        assert_eq!(player.sp_attack, 5.0);
        assert_eq!(player.attack_speed, 5.0);
        assert_eq!(player.defense, 5.0);
        assert_eq!(player.sp_defense, 5.0);
        assert_eq!(player.speed, 100.0);
        assert_eq!(player.health, 100.0);
        assert_eq!(player.size, Size::Normal);
        assert_eq!(player.pos, Vec3::ZERO);
    }
}
