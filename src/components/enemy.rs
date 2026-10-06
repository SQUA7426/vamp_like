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
            .add_systems(Update, chase_player.run_if(in_state(GameState::Playing)));
    }
}

fn setup_enemy_resources(mut cmds: Commands) {
    cmds.insert_resource(EnemyDecayRate(1.0));
    cmds.insert_resource(EnemyMaxCount(1));
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

    if (enemies.iter().len() as i32) > max_enemies.0 {
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
    enemy_decay_rate: Option<Res<EnemyDecayRate>>,
    time: Res<Time>,
) {
    let Some(mut enemy_query) =  enemy_query else { return };
    let Some(enemy_decay_rate) = enemy_decay_rate else { return };

    let player_transform = player.into_inner();

    for (enemy, mut enemy_transform) in &mut enemy_query {
        let delta_time = time.delta_secs();

        let diff_translation = player_transform.translation - enemy_transform.translation;

        let movement = diff_translation * enemy.speed;

        enemy_transform.translation.smooth_nudge(&movement, enemy_decay_rate.0, delta_time);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_single_enemy() {
        let enemy = Enemy::new(10.0);

        assert_eq!(enemy.speed, 110.0);
        assert_eq!(enemy.health, 10.0);
        assert_eq!(enemy.size, Size::Normal);
        assert_eq!(enemy.pos, Vec2::ZERO);
    }
}
