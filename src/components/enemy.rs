use std::f32::consts::PI;

use crate::{
    components::{
        level::{PlayingDecayRate, setup_level_ui},
        menu::GameState,
        player::Player,
        size::Size,
    },
    traits::character::Character,
};
use bevy::{color::palettes::css::CRIMSON, math::ops::sqrt, prelude::*};
use rand::random;

#[derive(Resource, Debug)]
pub struct EnemyMaxCount(i32);

#[derive(Resource, Debug)]
pub struct EnemySpawnTimer(Timer);

#[allow(unused)]
#[derive(Component, Debug)]
pub struct Enemy {
    pub name: String,
    speed: f32,
    pub health: f32,
    max_health: f32,
    size: Size,
    pos: Vec2,
    pub drop_exp: f32,
}

impl Character for Enemy {
    fn new(char_name: String, hp: f32) -> Self {
        Self {
            name: char_name,
            speed: 70.0,
            health: hp,
            max_health: hp,
            size: Size::default(),
            pos: Vec2::ZERO,
            drop_exp: 2.5
        }
    }

    fn spawnpoint(&self, player_pos: Vec3) -> Vec3 {
        let r = 400.0;
        let theta: f32 = random::<f32>() * 2.0 * PI;

        let e_x = player_pos.x + r * f32::cos(theta);
        let e_y = player_pos.y + r * f32::sin(theta);

        Vec3 {
            x: e_x,
            y: e_y,
            z: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            setup_enemy_resources.after(setup_level_ui),
        )
        .add_systems(
            Update,
            (enemy_can_spawn, spawn_enemies)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (enemy_died, chase_player, despawn_on_player_pos).run_if(in_state(GameState::Playing)),
        );
    }
}

fn setup_enemy_resources(mut cmds: Commands, playing_decay_rate: Res<PlayingDecayRate>) {
    cmds.insert_resource(EnemyMaxCount(10));
    let decay_rate = playing_decay_rate.into_inner();
    cmds.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        2.5 / decay_rate.rate,
        TimerMode::Repeating,
    )));
}

fn enemy_can_spawn(
    mut cmds: Commands,
    enemy_query: Option<Query<&Enemy>>,
    max_enemies: Option<Res<EnemyMaxCount>>,
) {
    let Some(enemies) = enemy_query else { return };

    let Some(max_enemies) = max_enemies else {
        return;
    };

    if (enemies.iter().len() as i32) == max_enemies.0 {
        cmds.remove_resource::<EnemyMaxCount>();
    } else {
        cmds.insert_resource(EnemyMaxCount(10));
    }
}

fn spawn_enemies(
    mut cmds: Commands,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    player: Single<&Transform, With<Player>>,
    max_enemies: Option<Res<EnemyMaxCount>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !enemy_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let Some(_max_enemies) = max_enemies else {
        return;
    };

    let player_pos = player.into_inner();
    let enemy = Enemy::new("Dummy".to_string(), 10.0);
    let spawn_pt = enemy.spawnpoint(player_pos.translation);

    cmds.spawn((
        DespawnOnExit(GameState::Playing),
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(Color::from(CRIMSON))),
        Transform::from_xyz(spawn_pt.x, spawn_pt.y, 100.0),
        enemy,
    ));
}

fn enemy_died(
    mut cmds: Commands,
    enemy_query: Option<Query<(Entity, &Enemy)>>,
    player: Single<&mut Player>
) {
    let Some(enemy_query) = enemy_query else { return };
    let mut player = player.into_inner();

    for (entity, enemy) in &enemy_query {
        if enemy.health <= 0.0 {
            player.exp += enemy.drop_exp;
            cmds.entity(entity).despawn();
        }
    }
}

fn chase_player(
    enemy_query: Option<Query<(&Enemy, &mut Transform)>>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
    playing_decay_rate: Res<PlayingDecayRate>,
    time: Res<Time>,
) {
    let Some(mut enemy_query) = enemy_query else {
        return;
    };

    let player_transform = player.into_inner();
    let decay_rate = playing_decay_rate.into_inner();

    for (enemy, mut enemy_transform) in &mut enemy_query {
        let diff_translation = player_transform.translation - enemy_transform.translation;

        let movement_delta = diff_translation.normalize_or_zero()
            * enemy.speed
            * time.delta_secs()
            * decay_rate.rate;

        enemy_transform.translation += movement_delta;
    }
}

fn despawn_on_player_pos(
    mut cmds: Commands,
    enemy_query: Option<Query<(Entity, &Enemy, &Transform)>>,
    player: Single<(&Transform, &mut Player)>,
) {
    let Some(mut enemy_query) = enemy_query else {
        return;
    };

    let (player_transform, mut player) = player.into_inner();

    for (entity, enemy, enemy_transform) in &mut enemy_query {
        if enemy_near_player(enemy_transform.translation, 15.0, player_transform.translation, 24.0) {
            player.health -= enemy.health;
            player.exp += enemy.drop_exp;
            println!("Deleting Enemy...");
            cmds.entity(entity).despawn();
        }
    }
}

pub fn enemy_near_player(enemy_pos: Vec3, e_radius: f32, player_pos: Vec3, p_radius: f32) -> bool {
    let e_x = enemy_pos.x;
    let e_y = enemy_pos.y;
    let p_x = player_pos.x;
    let p_y = player_pos.y;

    let distance: f32 = sqrt((e_x - p_x).powi(2) + (e_y - p_y).powi(2));
    distance >= f32::abs(e_radius - p_radius) && distance <= e_radius + p_radius
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Player;
    use crate::type_of;

    #[test]
    fn setup_single_enemy() {
        let enemy = Enemy::new("Dummy".to_string(), 10.0);

        assert_eq!(enemy.name, String::from("Dummy"));
        assert_eq!(enemy.speed, 70.0);
        assert_eq!(enemy.health, 10.0);
        assert_eq!(enemy.max_health, 10.0);
        assert_eq!(enemy.size, Size::Normal);
        assert_eq!(enemy.pos, Vec2::ZERO);
        assert_eq!(enemy.pos, Vec2::ZERO);
        assert_eq!(enemy.drop_exp, 2.5);
    }

    #[test]
    fn test_enemy_near_player() {
        let enemy = Enemy::new("Dummy".to_string(), 10.0);
        let enemy_pos = enemy.pos.extend(0.0);

        let player_pos = Vec3 {
            x: 20.0,
            y: 50.0,
            z: 0.0,
        };

        assert_eq!(enemy_near_player(enemy_pos, 15.0, player_pos, 24.0), false);
    }

    #[test]
    fn enemy_spawn_pt() {
        let enemy = Enemy::new("Dummy".to_string(), 10.0);

        let player = Player::new("Anton".to_string(), 100.0);
        let spawn_pt = enemy.spawnpoint(player.pos);

        assert_eq!(type_of(&spawn_pt), "glam::f32::vec3::Vec3");
    }
}
