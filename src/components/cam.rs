use bevy::prelude::*;

use crate::components::player::Player;

#[derive(Resource)]
pub struct DecayRate(f32);

#[derive(Debug)]
pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cam)
            .add_systems(Update, update_cam);
    }
}

fn setup_cam(mut cmds: Commands) {
    cmds.insert_resource(DecayRate(2.0));

    // Technically OrthographicProjection::default_2d() with another zoom
    let cam_proj = Projection::Orthographic(OrthographicProjection {
        near: -1000.0,
        far: 1000.0,
        viewport_origin: vec2(0.5, 0.5),
        scaling_mode: default(),
        scale: 0.50, // for ZOOM
        area: Rect {
            min: vec2(-1.0, -1.0),
            max: vec2(1.0, 1.0),
        },
    });

    cmds.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        Transform::from_translation(vec3(0.0, 0.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
        cam_proj,
    ));
}

fn update_cam(
    mut cam: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    decay_rate: Res<DecayRate>,
    time: Res<Time<Fixed>>,
) {
    let decay_rate = decay_rate.0;

    let delta_time = time.delta_secs();

    let Vec3 { x, y, .. } = player.translation;

    let direction = Vec3::new(x, y, cam.translation.z);

    cam.translation
        .smooth_nudge(&direction, decay_rate, delta_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cam_plugin() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, CamPlugin)).update();
        assert!(app.is_plugin_added::<CamPlugin>());
    }
}
