#![allow(unused_imports)]
pub mod chain_structs;
pub mod cursor;

use anchor_lang::prelude::Pubkey;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_prototype_lyon::prelude::*;
use chain_structs::{
    mock_data, CFeature, CLocation, CMetadata, COccupant, COwner, TileEntity, TileMarker,
};
use dominarisystems::components::ComponentFeature;
use rand::Rng;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local!(static GLOBAL_MOVE_UP: RefCell<bool> = RefCell::new(false));

#[wasm_bindgen]
pub fn move_up() {
    GLOBAL_MOVE_UP.with(|text| *text.borrow_mut() = true);
}

pub fn create_components_from_chain_structs(mut commands: Commands) {
    let chain_ents = mock_data();

    info!("create system");
    for TileEntity {
        metadata,
        location,
        feature,
        owner,
        occupant,
    } in chain_ents.tiles
    {
        let entity = commands
            .spawn()
            .insert_bundle((
                TileMarker,
                CMetadata(metadata),
                CLocation(location),
                COwner(owner),
            ))
            .id();

        if let Some(feature) = feature {
            commands.entity(entity).insert(CFeature(feature));
        }
        if let Some(occupant) = occupant {
            commands.entity(entity).insert(COccupant(occupant));
        }
    }
}

pub fn spawn_render_tiles(
    mut commands: Commands,
    q: Query<(Entity, &TileMarker, &CLocation), Without<TilemapId>>,
    mut tile_map: Query<(Entity, &mut TileStorage)>,
) {
    let (tile_map, mut tile_storage) = tile_map.get_single_mut().unwrap();
    for (tile_entity, _, loc) in &q {
        let position = TilePos {
            x: loc.0.x as u32,
            y: loc.0.y as u32,
        };
        commands.entity(tile_entity).insert_bundle(TileBundle {
            position,
            tilemap_id: TilemapId(tile_map),
            ..default()
        });
        tile_storage.set(&position, tile_entity);
    }
}