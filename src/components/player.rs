use bevy::prelude::*;

use crate::components::animation::{AnimationConfig, PlayerSprite};
use crate::components::enemy::{Enemy, enemy_near_player};
use crate::components::level::{UiComponent, UiTextComponent, setup_level_ui};
use crate::{
    components::{
        level::PlayingDecayRate,
        menu::{GameState, MenuState},
        size::Size,
    },
    traits::character::Character,
};

#[derive(Resource)]
pub struct PlayerAttackSpeed(Timer);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum PlayerState {
    #[default]
    Standing,
    Running,
    Attacking,
    Damaging,
}

#[allow(unused)]
#[derive(Component, Debug, Clone)]
pub struct Player {
    name: String,
    pub lvl: i32,
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
    state: PlayerState,
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
            state: PlayerState::default(),
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
        app.add_systems(
            OnEnter(GameState::Playing),
            (player_setup, setup_healthtext)
                .chain()
                .after(setup_level_ui),
        )
        .add_systems(
            Update,
            (attack_enemy, control_player, update_player_ui).run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, exec_animation)
        .add_systems(Update, trigger_sprite::<PlayerSprite>)
        .add_systems(Update, level_up.run_if(in_state(GameState::Playing)))
        .add_systems(Update, player_died.run_if(in_state(GameState::Playing)));
    }
}

fn setup_healthtext(ui_query: Query<(&UiTextComponent, &mut Text)>, player: Single<&Player>) {
    let player = player.into_inner();
    for (ui, mut txt) in ui_query {
        if *ui == UiTextComponent::HP {
            *txt = Text::new(format!("{:?}/{:?}", player.health, player.max_health));
        }
    }
}

fn player_setup(
    mut cmds: Commands,
    playing_decay_rate: Res<PlayingDecayRate>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player = Player::new("Anton".to_string(), 100.0);

    let player_pos = player.spawnpoint(player.pos);
    let decay_rate = playing_decay_rate.into_inner();

    let mut player_transform = Transform::from_translation(player_pos);

    player_transform = player_transform.with_scale(vec3(2.0, 2.0, 0.0));

    let texture: Handle<Image> = asset_server.load("textures/player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 5, 2, None, None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_conf = AnimationConfig::new(1, 4, 8 * decay_rate.rate as u8);

    cmds.spawn((
        DespawnOnExit(GameState::Playing),
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_conf.first_sprite_idx,
            }),
            ..Default::default()
        },
        player_transform,
        player.clone(),
        PlayerSprite,
        animation_conf,
        PlayerState::Standing,
    ));

    cmds.insert_resource(PlayerAttackSpeed(Timer::from_seconds(
        player.attack_speed / decay_rate.rate,
        TimerMode::Repeating,
    )));
}

fn control_player(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Player, &mut Transform, &mut Sprite, &mut PlayerState)>,
    playing_decay_rate: Res<PlayingDecayRate>,
    time: Res<Time>,
) {
    let (mut player, mut transform, mut sprite, mut player_state) = player.into_inner();
    let decay_rate = playing_decay_rate.into_inner();

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
        *player_state = PlayerState::Running;
    }

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
        *player_state = PlayerState::Running;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
        sprite.flip_x = true;
        *player_state = PlayerState::Running;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
        sprite.flip_x = false;
        *player_state = PlayerState::Running;
    }

    let movement_delta =
        direction.normalize_or_zero() * player.speed * time.delta_secs() * decay_rate.rate;
    transform.translation += movement_delta.extend(0.0);

    player.pos = transform.translation;
}

fn update_player_ui(
    mut ui_text_query: Query<(&UiTextComponent, &mut Text)>,
    mut ui_query: Query<(&UiComponent, &mut Node, &mut ImageNode)>,
    player: Single<(&Player, &mut PlayerState)>,
) {
    let (player, mut player_state) = player.into_inner();
    for (ui_text, mut txt) in &mut ui_text_query {
        match *ui_text {
            UiTextComponent::Level => {
                *txt = Text::new(format!("lvl: {:?}", player.lvl));
            }
            UiTextComponent::HP => {
                if *player_state == PlayerState::Damaging {
                    *txt = Text::new(format!("{:?}/{:?}", player.health, player.max_health));
                    *player_state = PlayerState::Standing;
                }
            }
            UiTextComponent::Exp => {
                *txt = Text::new(format!("{:?}/{:?}", player.exp, player.exp_max));
            }
            _ => {}
        }
    }

    for (ui, mut node, mut image_node) in &mut ui_query {
        if *ui == UiComponent::HP {
            let percentage = (player.health / player.max_health) * 100.0;
            let curr_color: Color = if percentage > 45.0 {
                Color::srgb(0.1, 1.0, 0.0)
            } else if percentage <= 45.0 && percentage > 25.0 {
                Color::srgb(0.4, 0.9, 0.0)
            } else if percentage <= 25.0 && percentage > 10.0 {
                Color::srgb(0.7, 0.5, 0.0)
            } else {
                Color::srgb(0.6, 0.0, 0.0)
            };
            let new_image_node = image_node.clone().with_color(curr_color);
            *image_node = new_image_node;
            *node = Node {
                height: percent(100.0),
                width: percent(percentage),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            };
        }
    }
}

fn attack_enemy(
    enemy_query: Option<Query<(&mut Enemy, &Transform)>>,
    player: Single<(&mut Player, &Transform, &Sprite, &mut AnimationConfig)>,
    mut attack_speed: ResMut<PlayerAttackSpeed>,
    playing_decay_rate: Res<PlayingDecayRate>,
    time: Res<Time>,
) {
    let Some(mut enemy_query) = enemy_query else {
        return;
    };
    let (player, player_transform, sprite, mut animation_conf) = player.into_inner();

    let texture_atlas = sprite.texture_atlas.clone().unwrap();

    let decay_rate = playing_decay_rate.into_inner();
    if texture_atlas.index == 0 || texture_atlas.index == 9 {
        *animation_conf = AnimationConfig::new(1, 4, 8 * decay_rate.rate as u8);
    }

    if !attack_speed.0.tick(time.delta()).just_finished() {
        return;
    }

    for (mut enemy, enemy_transform) in &mut enemy_query {
        if enemy_near_player(
            enemy_transform.translation,
            15.0,
            player_transform.translation,
            player.attack_range,
        ) {
            *animation_conf = AnimationConfig::new(5, 9, 8 * decay_rate.rate as u8);
            enemy.health -= player.attack;
            return;
        }
    }
}

fn level_up(player: Single<&mut Player>) {
    let mut player = player.into_inner();

    if player.exp >= player.exp_max {
        player.exp -= player.exp_max;
        player.lvl += 1;
        player.exp_max *= 1.25;
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

fn exec_animation(mut query: Query<(&mut AnimationConfig, &mut Sprite)>, time: Res<Time>) {
    query.iter_mut().for_each(|(mut conf, mut sprite)| {
        conf.frame_timer.tick(time.delta());

        if conf.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == 0 {
                conf.first_sprite_idx = 1;
                conf.last_sprite_idx = 4;
                atlas.index += 1;
                conf.is_playing = true;
                conf.frame_timer = AnimationConfig::timer_from_fps(conf.fps);
            } else if atlas.index == conf.last_sprite_idx {
                atlas.index = conf.first_sprite_idx;
                conf.is_playing = true;
            } else {
                atlas.index += 1;
                conf.frame_timer = AnimationConfig::timer_from_fps(conf.fps);
            }
        }
    });
}

fn trigger_sprite<C: Component>(mut _query: Single<(&mut AnimationConfig, &mut Sprite), With<C>>) {
    let (mut conf, mut sprite) = _query.into_inner();

    if !conf.is_playing {
        conf.is_playing = true;
        conf.frame_timer.reset();

        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = conf.first_sprite_idx;
        }
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
        assert_eq!(player.state, PlayerState::default());
    }
}
