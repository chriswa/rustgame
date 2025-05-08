use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_ecs_tilemap::prelude::*;

// Constants for tile and atlas configuration
pub const TILE_SIZE: u32 = 16;
pub const ATLAS_SIZE: u32 = 16; // Number of tiles in each dimension of the atlas

// Colors for our generated tiles
const TILE_COLORS: [Color; 3] = [
    Color::srgb(0.2, 0.1, 0.1), // Red
    Color::srgb(0.1, 0.2, 0.1), // Green
    Color::srgb(0.1, 0.1, 0.2), // Blue
];

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Create a new image for our tile atlas
    let mut atlas_image = Image::new(
        Extent3d {
            width: TILE_SIZE * ATLAS_SIZE,
            height: TILE_SIZE * ATLAS_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![
            0;
            TILE_SIZE as usize * ATLAS_SIZE as usize * TILE_SIZE as usize * ATLAS_SIZE as usize * 4
        ],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );

    // Fill the first 3 tiles with solid colors
    for i in 0..3 {
        let color = TILE_COLORS[i];
        let start_x = (i as u32 * TILE_SIZE) as usize;
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                let pixel_index =
                    (y as usize * ATLAS_SIZE as usize * TILE_SIZE as usize + x as usize + start_x)
                        * 4;
                if let Some(data) = atlas_image.data.as_mut() {
                    let [r, g, b, a] = color.to_srgba().to_f32_array();
                    data[pixel_index] = (r * 255.0) as u8;
                    data[pixel_index + 1] = (g * 255.0) as u8;
                    data[pixel_index + 2] = (b * 255.0) as u8;
                    data[pixel_index + 3] = (a * 255.0) as u8;
                }
            }
        }
    }

    // Add the image to the asset server
    let atlas_handle = images.add(atlas_image);

    // Create the tilemap
    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // Fill the tilemap with some pattern
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex((x + y) % 3), // Cycle through our 3 tiles
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Create the tilemap bundle
    let tile_size = TilemapTileSize {
        x: TILE_SIZE as f32,
        y: TILE_SIZE as f32,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(atlas_handle),
        tile_size,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        // anchor: TilemapAnchor::default(),
        ..default()
    });
}
