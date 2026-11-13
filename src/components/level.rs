use bevy::{
    color::palettes::css::BLACK,
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::{components::menu::GameState, create_node};

#[derive(Resource, Debug)]
pub struct PlayingDecayRate {
    pub rate: f32,
}

#[derive(Debug)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_floor, setup_level_ui));
    }
}

fn setup_floor(mut cmds: Commands, assets: Res<AssetServer>) {
    let chunk_size = UVec2::splat(32);
    let tile_display_size = UVec2::splat(64);

    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|i| Some(TileData::from_tileset_index(i as u16)))
        .collect();

    cmds.spawn((
        DespawnOnExit(GameState::Playing),
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets.load_with_settings(
                "textures/floor_texture.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 2 });
                },
            ),
            ..default()
        },
        TilemapChunkTileData(tile_data.clone()),
        Transform::from_translation(Vec3::new(0., 0., -200.)),
    ));
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UiComponent {
    Profile,
    Pause,
    Level,
    HP,
    Exp,
    Cards,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UiTextComponent {
    Profile,
    Pause,
    Level,
    HP,
    Exp,
    Cards,
}

pub fn setup_level_ui(mut cmds: Commands, asset_server: Res<AssetServer>) {
    [
        (
            5.0,
            85.0,
            5.0,
            Some(85.0),
            10.0,
            10.0,
            UiComponent::Profile,
            UiTextComponent::Profile,
            "Profile",
        ), // PROF PIC
        (
            85.0,
            5.0,
            5.0,
            Some(85.0),
            10.0,
            10.0,
            UiComponent::Pause,
            UiTextComponent::Pause,
            "Pause",
        ), // Pause
        (
            42.5,
            42.5,
            1.5,
            Some(95.5),
            15.0,
            3.0,
            UiComponent::Level,
            UiTextComponent::Level,
            "LVL",
        ), // LVL
        (
            30.0,
            30.0,
            5.5,
            Some(89.5),
            40.0,
            5.0,
            UiComponent::HP,
            UiTextComponent::HP,
            "100.0/100.0",
        ), // HP
        (
            30.0,
            30.0,
            11.5,
            Some(83.5),
            40.0,
            3.0,
            UiComponent::Exp,
            UiTextComponent::Exp,
            "EXP",
        ), // Exp
        (
            25.0,
            25.0,
            79.0,
            Some(5.0),
            50.0,
            16.0,
            UiComponent::Cards,
            UiTextComponent::Cards,
            "CARDS",
        ), // Cards
    ]
    .into_iter()
    .for_each(|(l, r, t, b, w, h, ui, ui_text, txt)| {
        cmds.spawn((
            DespawnOnExit(GameState::Playing),
            create_node!(l, r, t, b, w, h),
            match ui {
                UiComponent::HP => BackgroundColor::from(Color::srgba(0.1, 0.8, 0.1, 0.0)),
                _ => BackgroundColor::from(Color::srgb(0.3, 0.3, 0.4)),
            },
            children![(
                Node {
                    height: percent(150.0),
                    width: percent(70.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                match ui {
                    UiComponent::HP => ImageNode {
                        image: asset_server.load("ui/health_bar_bar.png"),
                        ..default()
                    },
                    _ => ImageNode::default(),
                },
                children![(
                    ui,
                    Node {
                        height: percent(100.0),
                        width: percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    match ui {
                        UiComponent::HP => ImageNode {
                            image: asset_server.load("ui/health_bar_health.png"),
                            ..default()
                        },
                        _ => ImageNode::default(),
                    },
                    children![(
                        ui_text,
                        match ui {
                            UiComponent::HP => Node {
                                left: percent(5.0),
                                ..default()
                            },
                            _ => Node { ..default() },
                        },
                        Text::new(txt),
                        TextColor::from(BLACK),
                    )],
                )],
            )],
        ));
    });

    cmds.insert_resource(PlayingDecayRate { rate: 4.0 });
}
