use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

// Constants for tile and atlas configuration
pub const TILE_SIZE: u32 = 16;
pub const ATLAS_SIZE: u32 = 16; // Number of tiles in each dimension of the atlas

// Colors for our generated tiles
const TILE_COLORS: [Color; 3] = [
    Color::srgb(0.2, 0.1, 0.1), // Red
    Color::srgb(0.1, 0.2, 0.1), // Green
    Color::srgb(0.1, 0.1, 0.2), // Blue
];

#[derive(Resource)]
pub struct Tileset {
    pub atlas_handle: Handle<Image>,
    pub tile_size: u32,
    pub texture_index_count: u32,
}

impl Tileset {
    pub fn new(images: &mut Assets<Image>) -> Self {
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
                TILE_SIZE as usize
                    * ATLAS_SIZE as usize
                    * TILE_SIZE as usize
                    * ATLAS_SIZE as usize
                    * 4
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
                    let pixel_index = (y as usize * ATLAS_SIZE as usize * TILE_SIZE as usize
                        + x as usize
                        + start_x)
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

        Self {
            atlas_handle,
            tile_size: TILE_SIZE,
            texture_index_count: ATLAS_SIZE * ATLAS_SIZE,
        }
    }
}
