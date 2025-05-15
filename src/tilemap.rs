use crate::tileset::Tileset;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct Tilemap {
    entity: Entity,
    storage: TileStorage,
    map_size: TilemapSize,
}

impl Tilemap {
    pub fn new(
        commands: &mut Commands,
        tileset: &Tileset,
        map_size: TilemapSize,
        texture_indices: impl Iterator<Item = u32>,
        transform: Transform,
    ) -> Self {
        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(map_size);

        // Create tiles based on the provided iterator
        for (i, texture_index) in texture_indices.enumerate() {
            assert!(texture_index < tileset.texture_index_count);
            let x = (i as u32) % map_size.x;
            let y = (i as u32) / map_size.x;
            if x < map_size.x && y < map_size.y {
                let tile_pos = TilePos { x, y };
                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(texture_index),
                        ..default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        // Create the tilemap bundle
        let tile_size = TilemapTileSize {
            x: tileset.tile_size as f32,
            y: tileset.tile_size as f32,
        };
        let grid_size = tile_size.into();
        let map_type = TilemapType::default();

        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage.clone(),
            texture: TilemapTexture::Single(tileset.atlas_handle.clone()),
            tile_size,
            transform,
            ..default()
        });

        Self {
            entity: tilemap_entity,
            storage: tile_storage,
            map_size,
        }
    }

    pub fn update_tile(
        &self,
        commands: &mut Commands,
        position: TilePos,
        texture_index: u32,
    ) -> bool {
        if let Some(tile_entity) = self.storage.get(&position) {
            commands
                .entity(tile_entity)
                .insert(TileTextureIndex(texture_index));
            true
        } else {
            false
        }
    }
}
