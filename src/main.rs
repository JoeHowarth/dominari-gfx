use anchor_lang::solana_program::pubkey::Pubkey;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_prototype_lyon::prelude::*;
use dominari_gfx::{setup, update_path_closest_neighboor, TravelPath};
use dominarisystems::components::{
    ComponentFeature, ComponentLocation, ComponentMapMeta, ComponentMetadata, ComponentOccupant,
    ComponentOwner,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            // width: 1000.,
            // height: 800.,
            title: "I am a window!".to_string(),
            #[cfg(target_arch = "wasm32")]
            canvas: Some(String::from(".game")),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(TilemapPlugin)
        .add_startup_system(spawn_empty_map) 
        .add_system(swap_texture_or_hide)
        .run();
}

fn spawn_empty_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let tilemap_size = TilemapSize { x: 8, y: 8 };
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let tilemap_type = TilemapType::Square {
        diagonal_neighbors: true,
    };

    let tilemap_entity = commands.spawn().id();

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16., y: 16.};
    let grid_size = tile_size.into();

    commands.entity(tilemap_entity).insert_bundle(TilemapBundle {
        grid_size,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 0.),
        ..default()
    });
}

fn swap_texture_or_hide(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let texture_a = TilemapTexture::Single(asset_server.load("tiles.png"));
        let texture_b = TilemapTexture::Single(asset_server.load("tiles2.png"));
        for (mut tilemap_tex, _) in &mut query {
            if *tilemap_tex == texture_a {
                *tilemap_tex = texture_b.clone();
            } else {
                *tilemap_tex = texture_a.clone();
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::H) {
        for (_, mut visibility) in &mut query {
            if visibility.is_visible {
                visibility.is_visible = false;
            } else {
                visibility.is_visible = true;
            }
        }
    }
}
