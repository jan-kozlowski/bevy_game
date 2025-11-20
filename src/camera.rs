use std::f32::consts::PI;

use bevy::{
    input::mouse::AccumulatedMouseMotion, 
    prelude::*,
    window::{CursorGrabMode, CursorOptions}
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {

    fn build(&self, app: &mut App) {
        
        app
            .add_systems(Startup, create_camera)
            .add_systems(Update, update_camera);

        println!("Camera plugin built successfully!");
    }
}

#[derive(Component)]
struct CameraController {

    movement_input: Vec2,
    camera_input: Vec2,
    sensitivity: f32,
    movement_speed: f32
}

impl Default for CameraController {

    fn default() -> Self {

        Self { 
            movement_input: Vec2::default(), 
            camera_input: Vec2::default(), 
            sensitivity: 0.1,
            movement_speed: 50.
        }
    }
}

fn create_camera(mut commands: Commands) {

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 5., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default()
    ));
}

fn update_camera(
    cameras: Single<(&mut CameraController, &mut Transform), With<Camera3d>>,
    windows: Query<(&Window, &mut CursorOptions)>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {

    for (window, mut cursor_options) in windows {

        cursor_options.grab_mode = if window.focused { CursorGrabMode::Locked } else { CursorGrabMode::None };
        cursor_options.visible = !window.focused;
    }

    const RADIANS_PER_DOT: f32 = 1. / 180.;
    let delta_time = time.delta_secs();

    let (mut controller, mut transform) = cameras.into_inner();

    // movement input
    controller.movement_input = Vec2::default();

    if keyboard_input.pressed(KeyCode::KeyD) {

        controller.movement_input.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {

        controller.movement_input.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {

        controller.movement_input.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {

        controller.movement_input.y -= 1.;
    }

    let forward = transform.forward() * controller.movement_input.y;
    let sideways = transform.right() * controller.movement_input.x;
    let movement = (forward + sideways).normalize_or_zero() * controller.movement_speed * delta_time;
    transform.translation += movement;

    // camera input
    controller.camera_input.x = 
        (controller.camera_input.x - mouse_motion.delta.y * RADIANS_PER_DOT * controller.sensitivity)
        .clamp(-PI / 2., PI / 2.);
    controller.camera_input.y -=
        mouse_motion.delta.x * RADIANS_PER_DOT * controller.sensitivity;
    transform.rotation = Quat::from_euler(EulerRot::ZYX, 0., controller.camera_input.y, controller.camera_input.x);
}