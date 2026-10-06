use bevy::math::ops::sqrt;
use bevy::prelude::*;
use bevy::color::palettes::css::CRIMSON;
use crate::components::menu::GameState;
use crate::components::player::Player;
use crate::components::size::Size;

#[allow(unused)]
#[derive(Resource, Debug)]
pub struct EnemyMaxCount(i32);

#[allow(unused)]
#[derive(Resource, Debug)]
pub struct EnemySpawnTimer(Timer);

#[allow(unused)]
#[derive(Resource, Debug)]
pub struct EnemyDecayRate(f32);

#[allow(unused)]
#[derive(Component,Debug)]
pub struct Enemy {
    speed: f32,
    health: f32,
    size: Size,
    pos: Vec2,
}

impl Enemy {
    fn new(hp: f32) -> Self {
        Self {
            speed: 110.0,
            health: hp,
            size: Size::default(),
            pos: Vec2::ZERO,
        }
    }
}

#[derive(Debug)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), setup_enemy_resources)
            .add_systems(Update, spawn_enemies.run_if(in_state(GameState::Playing)))
            .add_systems(Update, (chase_player, despawn_on_player_pos).run_if(in_state(GameState::Playing)));
    }
}

fn setup_enemy_resources(mut cmds: Commands) {
    cmds.insert_resource(EnemyDecayRate(2.0));
    cmds.insert_resource(EnemyMaxCount(10));
    cmds.insert_resource(EnemySpawnTimer(Timer::from_seconds(2.5, TimerMode::Repeating)));
}

fn spawn_enemies(
    mut cmds: Commands,
    enemy_query: Option<Query<&Enemy>>,
    max_enemies: Res<EnemyMaxCount>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let Some(enemies) = enemy_query else { return };

    if !enemy_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    if (enemies.iter().len() as i32) == max_enemies.0 {
        return
    }
    let enemy = Enemy::new(10.0);

    cmds.spawn((
            Mesh2d(meshes.add(Circle::new(15.0))),
            MeshMaterial2d(materials.add(Color::from(CRIMSON))),
            Transform::from_xyz(enemy.pos.x, enemy.pos.y, 100.0),
            enemy,
    ));
}

fn chase_player(
    enemy_query: Option<Query<(&Enemy, &mut Transform)>>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Some(mut enemy_query) =  enemy_query else { return };

    let player_transform = player.into_inner();

    for (enemy, mut enemy_transform) in &mut enemy_query {

        let diff_translation = player_transform.translation - enemy_transform.translation;

        let movement_delta = diff_translation.normalize_or_zero() * enemy.speed * time.delta_secs();

        enemy_transform.translation += movement_delta;
    }
}

fn despawn_on_player_pos(
    mut cmds: Commands,
    enemy_query: Option<Query<(Entity, &Enemy, &Transform)>>,
    player: Single<&Transform, With<Player>>,
) {
    let Some(mut enemy_query) =  enemy_query else { return };

    let player_transform = player.into_inner();

    for (entity, _enemy, enemy_transform) in &mut enemy_query {
        if enemy_near_player(enemy_transform.translation, player_transform.translation) {
            println!("Deleting Enemy...");
            cmds.entity(entity).despawn();
        }
    }
}

fn enemy_near_player(enemy_pos: Vec3, player_pos: Vec3) -> bool {
    let e_x = enemy_pos.x;
    let e_y = enemy_pos.y;
    let e_radius = 15.0;
    let p_x = player_pos.x;
    let p_y = player_pos.y;
    let p_radius = 24.0;

    let distance: f32 = sqrt((e_x-p_x).powi(2) + (e_y-p_y).powi(2));
    distance >= f32::abs(e_radius-p_radius) && distance <= e_radius + p_radius
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Player;

    #[test]
    fn setup_single_enemy() {
        let enemy = Enemy::new(10.0);

        assert_eq!(enemy.speed, 110.0);
        assert_eq!(enemy.health, 10.0);
        assert_eq!(enemy.size, Size::Normal);
        assert_eq!(enemy.pos, Vec2::ZERO);
    }

    #[test]
    fn test_enemy_near_player() {
        let enemy = Enemy::new(10.0);
        let enemy_pos = enemy.pos.extend(0.0);

        let player = Player::new("Anton".to_string());
        let mut player_pos = player.pos.extend(0.0);
        player_pos += Vec3 {x: 20.0, y: 50.0, z: 0.0};

        assert_eq!(enemy_near_player(enemy_pos, player_pos), false);
    }
}
