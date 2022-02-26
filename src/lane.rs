use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use rand::{prelude::SliceRandom, Rng};

pub struct LaneCount(pub isize);

#[derive(Debug, Clone, Copy)]
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
pub struct Lane {
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
    let states = vec![LaneState::Entering, LaneState::Leaving];
    let mut rng = rand::thread_rng();
    let screen_bottom_left = Vec3::new(-(window.width / 2.), -(window.height / 2.), 5.);

    // Insert a new lane for each of these
    for lane_c in 0..lane_count.0 {
        let lane = Lane {
            width: lane_width,
            height: lane_height,
            state: *states.choose(&mut rng).unwrap(),
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

pub fn update_lane_height_on_update(
    mut lanes: Query<(&Lane, &mut Path, &mut Transform), Changed<Lane>>,
    window: Res<WindowDescriptor>,
) {
    for (lane, mut path, mut transform) in lanes.iter_mut() {
        let new_shape = shapes::Rectangle {
            extents: Vec2::new(lane.width, lane.height),
            origin: RectangleOrigin::BottomLeft,
        };

        // If the lane is leaving, we need to translate it to rest against the top of the screen
        match lane.state {
            LaneState::Entering => (),
            LaneState::OnScreen => (),
            LaneState::Leaving => transform.translation.y = (window.height / 2.) - lane.height,
        }

        *path = ShapePath::build_as(&new_shape);
    }
}

pub fn random_lane_height(mut lanes: Query<&mut Lane>, window: Res<WindowDescriptor>) {
    let mut rng = rand::thread_rng();
    for mut lane in lanes.iter_mut() {
        lane.height = window.height * rng.gen_range(0.2..=1.0)
    }
}
