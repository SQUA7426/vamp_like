use bevy::{
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

pub fn setup_level_ui(mut cmds: Commands) {
    [
        (5.0, 85.0, 5.0, Some(85.0), 10.0, 10.0),
        (85.0, 5.0, 5.0, Some(85.0), 10.0, 10.0),
        (45.0, 45.0, 1.5, Some(95.5), 10.0, 3.0),
        (30.0, 30.0, 5.5, Some(89.5), 40.0, 5.0),
        (25.0, 25.0, 79.0, Some(5.0), 50.0, 16.0),
    ]
    .into_iter()
    .for_each(|(l, r, t, b, w, h)| {
        cmds.spawn((
            DespawnOnExit(GameState::Playing),
            create_node!(l, r, t, b, w, h),
            BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 0.6)),
        ));
    });

    cmds.insert_resource(PlayingDecayRate { rate: 2.0});
}
