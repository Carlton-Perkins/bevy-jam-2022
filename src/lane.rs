use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use rand::prelude::SliceRandom;

pub struct LaneCount(pub isize);

#[derive(Debug)]
enum LaneState {
    Entering,
    OnScreen,
    Leaving,
}

impl Default for LaneState {
    fn default() -> Self {
        Self::Entering
    }
}

#[derive(Debug, Component, Default)]
struct Lane {
    width: f32,
    height: f32,
    state: LaneState,
}

#[derive(Bundle)]
struct LaneBundle {
    lane: Lane,

    #[bundle]
    shape: ShapeBundle,
}

pub fn lane_array_builder(
    mut c: Commands,
    window: Res<WindowDescriptor>,
    lane_count: Res<LaneCount>,
) {
    // Split the screen into lane_count columns
    let lane_width = window.width / lane_count.0 as f32;
    let lane_height = window.height;
    info!("Lane width is {}", lane_width);
    let colors = vec![
        Color::RED,
        Color::BLUE,
        Color::GREEN,
        Color::YELLOW,
        Color::PURPLE,
    ];
    let mut rng = rand::thread_rng();
    let screen_bottom_left = Vec3::new(-(window.width / 2.), -(window.height / 2.), 5.);

    // Insert a new lane for each of these
    for lane_c in 0..lane_count.0 {
        let lane = Lane {
            width: lane_width,
            height: lane_height,
            state: LaneState::OnScreen,
        };

        let shape = shapes::Rectangle {
            extents: Vec2::new(lane_width, lane_height),
            origin: RectangleOrigin::BottomLeft,
        };

        let geometry = GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(colors.choose(&mut rng).unwrap().clone())),
            Transform::from_translation(
                Vec3::new(lane_c as f32 * lane_width, 0., 5.) + screen_bottom_left,
            ),
        );

        c.spawn_bundle(LaneBundle {
            lane,
            shape: geometry,
        });
    }
}
