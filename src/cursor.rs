use bevy::{
    ecs::{component::SparseStorage, storage::SparseSet},
    math::Vec4Swizzles,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;

use crate::chain_structs::TileMarker;

#[derive(Default)]
pub struct CursorPos(pub Vec3);

pub struct CursorTilePos(pub TilePos);
pub struct CursorTileMarker;

impl Component for CursorTileMarker {
    type Storage = SparseStorage;
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
pub fn update_cursor_pos(
    windows: Res<Windows>,
    camera_q: Query<(&Transform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.iter() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            *cursor_pos = CursorPos(cursor_pos_in_world(
                &windows,
                cursor_moved.position,
                cam_t,
                cam,
            ));
        }
    }
}

// Converts the cursor position into a world position, taking into account any transforms applied
// the camera.
fn cursor_pos_in_world(
    windows: &Windows,
    cursor_pos: Vec2,
    cam_t: &Transform,
    cam: &Camera,
) -> Vec3 {
    let window = windows.primary();

    let window_size = Vec2::new(window.width(), window.height());

    // Convert screen position [0..resolution] to ndc [-1..1]
    // (ndc = normalized device coordinates)
    let ndc_to_world = cam_t.compute_matrix() * cam.projection_matrix().inverse();
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
    ndc_to_world.project_point3(ndc.extend(0.0))
}

pub fn update_cursor_tile_pos(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    cursor_tile: Query<Entity, (With<CursorTileMarker>, With<TileMarker>)>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
) {
    let (tm_size, tm_grid_size, tm_type, tm_storage, tm_transform) =
        tilemap_q.get_single().unwrap();
    let cursor_in_map_pos =
        (tm_transform.compute_matrix().inverse() * Vec4::from((cursor_pos.0, 1.))).xy();
    let Some(tile_pos) = TilePos::from_world_pos(&cursor_in_map_pos, tm_size, tm_grid_size, tm_type) else {
            debug!("Failed to get tile pos");
            return;
        };
    let Some(tile_entity) = tm_storage.get(&tile_pos) else {
            warn!("Boom1");
            return;
        };
    for tile in &cursor_tile {
        commands.entity(tile).remove::<CursorTileMarker>();
    }
    commands.entity(tile_entity).insert(CursorTileMarker);
}
