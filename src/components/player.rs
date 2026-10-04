use bevy::prelude::*;
use bevy::color::palettes::css::RED;

use crate::components::size::Size;

#[derive(Component)]
pub struct Player {
    name: String,
    health: f32,
    speed: f32,
    size: Size,
    pos: Vec2,
}

impl Player {
    fn new(name: String) -> Self {
        Self {
            name: name,
            speed: 100.0,
            health: 100.0,
            size: Size::default(),
            pos: Vec2::ZERO,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, control_player);
    }
}

fn player_setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let player = Player::new("Anton".to_string());
    cmds.spawn((
            Mesh2d(meshes.add(Circle::new(24.0))),
            MeshMaterial2d(materials.add(Color::from(RED))),
            Transform::from_xyz(player.pos.x, player.pos.y, 100.0),
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
        let player = Player::new("Anton".to_string());

        assert_eq!(player.name, "Anton".to_string());
        assert_eq!(player.speed, 100.0);
        assert_eq!(player.health, 100.0);
        assert_eq!(player.size, Size::Normal);
        assert_eq!(player.pos, Vec2::ZERO);
    }
}
