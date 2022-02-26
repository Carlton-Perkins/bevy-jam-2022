mod lane;

use bevy::{prelude::*, window::WindowResizeConstraints};
use bevy_prototype_lyon::prelude::*;

const SCREEN_WIDTH: f32 = 1500.;
const SCREEN_HEIGHT: f32 = 1100.;

fn main() {
    App::new()
        .insert_resource(lane::LaneCount(5))
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resize_constraints: WindowResizeConstraints {
                min_width: SCREEN_WIDTH,
                min_height: SCREEN_HEIGHT,
                max_width: SCREEN_WIDTH,
                max_height: SCREEN_HEIGHT,
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(init)
        .add_startup_system(lane::lane_array_builder)
        .run()
}

fn init(mut c: Commands) {
    c.spawn_bundle(OrthographicCameraBundle::new_2d());
}
