use bevy::window::PrimaryWindow;
use bevy::{input::mouse::MouseWheel, prelude::*, sprite::MaterialMesh2dBundle};
use euclid::{Box2D, Point2D};
use std::ops::Mul;

/// PhantomData tag used to denote the i16 space in which the schematic exists
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldSpace;

#[derive(Component)]
struct MyCameraMarker;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct CursorWorldCoords(Vec2);

/// We will store the world position of the viewport bounding rect here.
#[derive(Resource, Default)]
struct VisibleWorldRect(Option<Box2D<f32, WorldSpace>>);

fn cursor_to_world(
    mut schematic_coords: ResMut<CursorWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MyCameraMarker>>,
) {
    if let Ok((camera, cam_transform)) = q_camera.get_single() {
        if let Ok(window) = q_window.get_single() {
            if let Some(coords) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(cam_transform, cursor))
            {
                schematic_coords.0 = coords;
            }
        }
    }
}

fn window_to_world(
    mut visible_coords: ResMut<VisibleWorldRect>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MyCameraMarker>>,
) {
    if let Ok((camera, cam_transform)) = q_camera.get_single() {
        if let Ok(window) = q_window.get_single() {
            // 0  1
            // 2  3
            let corners = [
                Vec2::ZERO,
                Vec2::new(window.width(), 0.),
                Vec2::new(0., window.height()),
                Vec2::new(window.width(), window.height()),
            ];
            let bb = corners.iter().filter_map(|p| {
                camera
                    .viewport_to_world_2d(cam_transform, *p)
                    .map(|v| Point2D::new(v.x, v.y))
            });
            // .map(|p| camera.viewport_to_world_2d(cam_transform, p));
            visible_coords.0 = Some(Box2D::from_points(bb));
            return;
        }
    }
    visible_coords.0 = None // if theres any problem, assume camera doesnt see anything
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.)).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}

fn camera_transform(
    mb: Res<ButtonInput<MouseButton>>,
    mut mm: EventReader<CursorMoved>,
    mut mw: EventReader<MouseWheel>,
    mut camera: Query<(&mut Transform, &MyCameraMarker)>,
) {
    if let Ok(mut cam) = camera.get_single_mut() {
        if mb.pressed(MouseButton::Middle) {
            let mut pan = Vec3::ZERO;
            for m in mm.read() {
                if let Some(d) = m.delta {
                    pan += Vec3::new(-d.x, d.y, 0.0);
                }
            }
            let t = cam.0.scale.mul(pan);
            cam.0.translation += t;
        }
        for mwe in mw.read() {
            cam.0.scale *= 1. - mwe.y / 10.
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup))
        .add_systems(Update, camera_transform)
        .add_systems(Update, window_to_world)
        .run();
}
