use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;

mod camera;
mod chunk;
use chunk::*;

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

    let chunk = Chunk::default();

    for x in 0..CHUNK_SIZE {

        for y in 0..CHUNK_SIZE {

            let column = chunk.blocks.get(x).unwrap();
            let row = column.get(y).unwrap();

            for z in 0..CHUNK_SIZE {

                let block = row & (1 << z);
                if block != 0 {

                    // cube
                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
                        MeshMaterial3d(materials.add(Color::linear_rgb(0., 1.0, 0.))),
                        Transform::from_xyz(x as f32, y as f32, z as f32),
                    ));
                }

                info!("x: {}, y: {}, z: {}", x, y, z);
            }
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