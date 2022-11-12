use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

thread_local!(static GLOBAL_MOVE_UP: RefCell<bool> = RefCell::new(false));

#[wasm_bindgen]
pub fn move_up() {
    GLOBAL_MOVE_UP.with(|text| *text.borrow_mut() = true);
}

#[derive(Component)]
pub struct Destination {
    id: Uuid,
}

#[derive(Component, Debug)]
pub struct TravelPath {
    pub path: Vec<Uuid>,
    pub current_pos: Vec2,
}

fn get_random_f32() -> f32 {
    return rand::thread_rng().gen_range(-400.0..400.0);
}

fn get_random_uuid() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

pub fn update_path_closest_neighboor(
    mut commands: Commands,
    query: Query<(&Destination, &Transform)>,
    mut path: ResMut<TravelPath>,
) {
    GLOBAL_MOVE_UP.with(|text| {
        if *text.borrow() {
            *text.borrow_mut() = false;
            let path = path.as_mut();
            let mut closest_uuid = get_random_uuid();
            let mut closest_distance = 1000.0;
            let mut closest_position = Vec2::default();
            for (dest, trans) in query.iter() {
                if !path.path.contains(&dest.id) {
                    let new_post_3d: Vec3 = trans.translation;
                    let new_pos = Vec2::new(new_post_3d.x, new_post_3d.y);
                    let distance_to_current_pos = path.current_pos.distance(new_pos);
                    if distance_to_current_pos < closest_distance {
                        closest_distance = distance_to_current_pos;
                        closest_uuid = dest.id;
                        closest_position = new_pos;
                    }
                }
            }

            let line = shapes::Line(path.current_pos, closest_position);

            commands.spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::BLACK, 5.0)),
                Transform::default(),
            ));

            path.current_pos = closest_position;
            path.path.push(closest_uuid);
            return;
        }
    });
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });

    commands.spawn_bundle(Camera2dBundle::default());
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(10.0),
        ..shapes::RegularPolygon::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ))
        .with_children(|parent| {
            for _ in 0..10 {
                let rnd_x = get_random_f32();
                let rnd_y = get_random_f32();
                let mut entity = parent.spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::CYAN),
                        outline_mode: StrokeMode::new(Color::BLACK, 10.0),
                    },
                    Transform::from_translation(Vec3::new(rnd_x, rnd_y, 0.0)),
                ));
                entity.insert(Destination {
                    id: get_random_uuid(),
                });
            }
        });
}
