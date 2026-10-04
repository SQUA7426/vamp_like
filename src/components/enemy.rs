use bevy::prelude::*;
use bevy::color::palettes::css::CRIMSON;
use crate::components::size::Size;

#[allow(unused)]
#[derive(Resource)]
pub struct EnemyMaxCount(i32);

#[allow(unused)]
#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

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
            .add_systems(Startup, setup_enemy_resources)
            .add_systems(Update, spawn_enemies);
    }
}

fn setup_enemy_resources(mut cmds: Commands) {
    cmds.insert_resource(EnemyMaxCount(1));
    cmds.insert_resource(EnemySpawnTimer(Timer::from_seconds(2.5, TimerMode::Repeating)));
}

fn spawn_enemies(
    mut cmds: Commands,
    enemy_query: Option<Query<&Enemy>>,
    max_enemies: Res<EnemyMaxCount>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let Some(enemies) = enemy_query else { return };

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
