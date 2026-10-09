use bevy::color::palettes::css::{BLUE_VIOLET, DARK_SLATE_GRAY};
use bevy::prelude::*;

use crate::components::enemy::{Enemy, enemy_near_player};
use crate::components::level::setup_level_ui;
use crate::{
    components::{
        level::PlayingDecayRate,
        menu::{GameState, MenuState},
        size::Size,
    },
    traits::character::Character,
};

#[derive(Component, Debug)]
pub struct PlayerHealthText;

#[derive(Resource)]
pub struct PlayerAttackSpeed(Timer);

#[allow(unused)]
#[derive(Component, Debug, Clone)]
pub struct Player {
    name: String,
    lvl: i32,
    pub exp: f32,
    exp_max: f32,
    attack: f32,
    pub attack_range: f32,
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
            lvl: 1,
            exp: 0.0,
            exp_max: 10.0,
            attack: 5.0,
            attack_range: 45.0,
            sp_attack: 5.0,
            attack_speed: 0.8,
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
}

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), player_setup.after(setup_level_ui))
            .add_systems(
                Update,
                (attack_enemy, control_player, update_health_text).run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, level_up.run_if(in_state(GameState::Playing)))
            .add_systems(Update, player_died.run_if(in_state(GameState::Playing)));
    }
}

fn player_setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    playing_decay_rate: Res<PlayingDecayRate>,
) {
    let player = Player::new("Anton".to_string(), 100.0);

    let player_pos = player.spawnpoint(player.pos);
    let decay_rate = playing_decay_rate.into_inner();

    let radius: f32 = 8.0;
    let len: f32 = 28.0;

    let mut transforming = Transform::from_translation(player_pos + vec3(30.0, -15.0, 0.0));
    transforming.rotation = Quat::from_rotation_z(f32::to_radians(-45.0));

    cmds.spawn((
        DespawnOnExit(GameState::Playing),
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
        DespawnOnExit(GameState::Playing),
        Text2d::new(format!("{:?}/{:?}", player.health, player.max_health)),
        Transform::from_translation(player_pos + vec3(0.0, 25.0, 100.0)),
        PlayerHealthText,
    ));

    cmds.insert_resource(PlayerAttackSpeed(Timer::from_seconds(player.attack_speed / decay_rate.rate, TimerMode::Repeating)));
}

fn control_player(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Player, &mut Transform), Without<PlayerHealthText>>,
    text: Single<(&PlayerHealthText, &mut Transform), Without<Player>>,
    playing_decay_rate: Res<PlayingDecayRate>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player.into_inner();
    let (_health_text, mut text_transform) = text.into_inner();
    let decay_rate = playing_decay_rate.into_inner();

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

    let movement_delta =
        direction.normalize_or_zero() * player.speed * time.delta_secs() * decay_rate.rate;
    transform.translation += movement_delta.extend(0.0);
    text_transform.translation += movement_delta.extend(0.0);

    transform.rotate_z(rot - player.rot);

    player.pos = transform.translation;
    player.rot = rot;
}

fn update_health_text(
    mut cmds: Commands,
    player: Single<&mut Player>,
    text: Single<(Entity, &PlayerHealthText), Without<Player>>,
) {
    let (entity, _text) = text.into_inner();
    cmds.entity(entity).insert(Text2d::new(format!(
        "{:?}/{:?}",
        player.health, player.max_health
    )));
}

fn attack_enemy(
    enemy_query: Option<Query<(&mut Enemy, &Transform)>>,
    player: Single<(&mut Player, &Transform)>,
    mut attack_speed: ResMut<PlayerAttackSpeed>,
    time: Res<Time>,
) {
    let Some(mut enemy_query) = enemy_query else { return };
    let (player, player_transform) = player.into_inner();

    if !attack_speed.0.tick(time.delta()).just_finished() {
        return;
    }

    // println!("PlayerAttackSpeed finished");

    for (mut enemy, enemy_transform) in &mut enemy_query {
        if enemy_near_player(enemy_transform.translation, 15.0, player_transform.translation, player.attack_range) {
            println!("{:?} near player!", enemy.name);
            enemy.health -= player.attack;
        }
    }
}

fn level_up(player: Single<&mut Player>) {
    let mut player = player.into_inner();

    if player.exp >= player.exp_max {
        player.exp -= player.exp_max;
        player.lvl += 1;
    }
}

fn player_died(
    player: Single<&mut Player>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    let player = player.into_inner();

    if player.health <= 0.0 {
        game_state.set(GameState::Menu);
        menu_state.set(MenuState::Home);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_player() {
        let player = Player::new("Anton".to_string(), 100.0);

        assert_eq!(player.name, "Anton".to_string());
        assert_eq!(player.lvl, 1);
        assert_eq!(player.exp, 0.0);
        assert_eq!(player.exp_max, 10.0);
        assert_eq!(player.attack, 5.0);
        assert_eq!(player.attack_range, 45.0);
        assert_eq!(player.sp_attack, 5.0);
        assert_eq!(player.attack_speed, 0.8);
        assert_eq!(player.defense, 5.0);
        assert_eq!(player.sp_defense, 5.0);
        assert_eq!(player.speed, 100.0);
        assert_eq!(player.health, 100.0);
        assert_eq!(player.max_health, 100.0);
        assert_eq!(player.size, Size::Normal);
        assert_eq!(player.pos, Vec3::new(0.0, 0.0, 100.0));
    }
}
