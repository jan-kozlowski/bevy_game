use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;

mod camera;

fn main() {

    App::new()
        .add_plugins((
            DefaultPlugins,
            camera::CameraPlugin,
            FpsOverlayPlugin {
                ..default()
            }
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>, mut commands: Commands) {

    for x in 1..5 {

        for y in 1..5 {

            // cube
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::linear_rgb(0., 1.0, 0.))),
                Transform::from_xyz(x as f32, y as f32, 0.),
            ));
        }
    }

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}