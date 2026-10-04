use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_floor);
    }
}

fn setup_floor(mut cmds: Commands, assets: Res<AssetServer>) {
    let chunk_size = UVec2::splat(32);
    let tile_display_size = UVec2::splat(64);

    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|i| Some(TileData::from_tileset_index(i as u16)))
        .collect();

    cmds.spawn(
        (
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
        ),
    );
}
