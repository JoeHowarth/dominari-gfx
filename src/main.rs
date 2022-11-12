use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use dominari_gfx::{setup, update_path_closest_neighboor, TravelPath};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1200.,
            height: 800.,
            title: "I am a window!".to_string(),
            #[cfg(target_arch = "wasm32")]
            canvas: Some(String::from(".game")),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(TravelPath {
            path: vec![],
            current_pos: Vec2::new(0.0, 0.0),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(update_path_closest_neighboor)
        .run();
}
