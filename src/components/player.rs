use bevy::color::palettes::css::{BLUE_VIOLET, DARK_SLATE_GRAY};
use bevy::prelude::*;

use crate::components::menu::GameState;
use crate::components::size::Size;
use crate::traits::character::Character;

#[derive(Component, Debug)]
pub struct PlayerHealthText;

#[allow(unused)]
#[derive(Component, Debug, Clone)]
pub struct Player {
    name: String,
    attack: f32,
    sp_attack: f32,
    attack_speed: f32,
    defense: f32,
    sp_defense: f32,
    speed: f32,
    pub health: f32,
    pub max_health: f32,
    size: Size,
    pub pos: Vec3,
    pub rot: f32,
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
            max_health: hp,
            size: Size::default(),
            pos: Vec3::new(0.0, 0.0, 100.0),
            rot: 0.0,
        }
    }

    fn spawnpoint(&self, player_pos: Vec3) -> Vec3 {
        player_pos
    }

    fn attack(&self) {}
}

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), player_setup)
            .add_systems(
                Update,
                (control_player, update_health_text).run_if(in_state(GameState::Playing)),
            );
    }
}

fn player_setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = Player::new("Anton".to_string(), 100.0);

    let player_pos = player.spawnpoint(player.pos);

    let radius: f32 = 8.0;
    let len: f32 = 28.0;

    let mut transforming = Transform::from_translation(player_pos + vec3(30.0, -15.0, 0.0));
    transforming.rotation = Quat::from_rotation_z(f32::to_radians(-45.0));

    cmds.spawn((
        Mesh2d(meshes.add(Circle::new(24.0))),
        MeshMaterial2d(materials.add(Color::from(BLUE_VIOLET))),
        Transform::from_translation(player_pos),
        player.clone(),
    ))
    .with_children(|parent| {
        parent.spawn((
            Mesh2d(meshes.add(Capsule2d::new(radius, len))),
            MeshMaterial2d(materials.add(Color::from(DARK_SLATE_GRAY))),
            transforming,
        ));
    });

    cmds.spawn((
        Text2d::new(String::from(format!(
            "{:?}/{:?}",
            player.health, player.max_health
        ))),
        Transform::from_translation(player_pos + vec3(0.0, 25.0, 100.0)),
        PlayerHealthText,
    ));
}

fn control_player(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Player, &mut Transform), Without<PlayerHealthText>>,
    text: Single<(&PlayerHealthText, &mut Transform), Without<Player>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player.into_inner();
    let (_health_text, mut text_transform) = text.into_inner();

    let mut direction = Vec2::ZERO;

    let mut rot = player.rot;

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
        rot = 180.0;
    }

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
        rot = 0.0;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
        rot = 90.0;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
        rot = -90.0;
    }

    let movement_delta = direction.normalize_or_zero() * player.speed * time.delta_secs();
    transform.translation += movement_delta.extend(0.0);
    text_transform.translation += movement_delta.extend(0.0);

    transform.rotate_z(rot - player.rot);

    player.rot = rot;
}

fn update_health_text(
    mut cmds: Commands,
    player: Single<&mut Player>,
    text: Single<(Entity, &PlayerHealthText), Without<Player>>,
) {
    let (entity, _text) = text.into_inner();
    cmds.entity(entity).insert(Text2d::new(String::from(format!(
        "{:?}/{:?}",
        player.health, player.max_health
    ))));
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
        assert_eq!(player.max_health, 100.0);
        assert_eq!(player.size, Size::Normal);
        assert_eq!(player.pos, Vec3::new(0.0, 0.0, 100.0));
    }
}
