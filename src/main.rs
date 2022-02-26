mod lane;

use bevy::{core::FixedTimestep, prelude::*, window::WindowResizeConstraints};
use bevy_prototype_lyon::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use heron::prelude::*;
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
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ShapePlugin)
        .add_startup_system(lane::lane_array_builder)
        .add_startup_system(setup_system)
        .add_system(keyboard_events)
        .run()
}

#[derive(Component)]
struct ExampleShape;

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::default(),
        ))
        .insert(ExampleShape)
        // Make it a rigid body
        .insert(RigidBody::Dynamic)

        // Attach a collision shape
        .insert(CollisionShape::Sphere { radius: 10.0 })

        // Optionally add other useful components...
        .insert(Velocity::from_linear(Vec3::X * 2.0))
        .insert(Acceleration::from_linear(Vec3::X * 1.0))
        .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
        .insert(RotationConstraints::lock());
}

fn keyboard_events(
    mut key_evr: EventReader<KeyboardInput>,
) {
    use bevy::input::ElementState;

    for ev in key_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
                if ev.key_code == Some(bevy::input::keyboard::KeyCode::W) {
                    // W is being held down
                    println!("Up");
                }
            }
            ElementState::Released => {
                println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}